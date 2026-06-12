use crate::config::AppConfig;
use sdl2::video::FullscreenType;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tetanes_core::prelude::*;

/// Duration a controller Start must be held to trigger emulator pause.
const LONG_PRESS_DURATION: Duration = Duration::from_secs(1);

/// How long the volume overlay stays visible after adjustment.
const VOLUME_DISPLAY_DURATION: Duration = Duration::from_millis(1500);

/// How long the save state overlay stays visible.
const STATE_DISPLAY_DURATION: Duration = Duration::from_secs(2);

/// Encapsulates all Sprint 4+5 runtime state: pause, fullscreen, volume, save state.
pub struct GameState {
    pub paused: bool,
    pub volume: u8,
    pub show_help: bool,
    // Controller Start long-press tracking
    controller_start_held_since: Option<Instant>,
    controller_pause_triggered: bool,
    // Volume overlay timer
    volume_overlay_until: Option<Instant>,
    // Save state overlay message + timer
    state_overlay_message: String,
    state_overlay_until: Option<Instant>,
}

impl GameState {
    pub fn new(volume: u8) -> Self {
        Self {
            paused: false,
            volume,
            show_help: false,
            controller_start_held_since: None,
            controller_pause_triggered: false,
            volume_overlay_until: None,
            state_overlay_message: String::new(),
            state_overlay_until: None,
        }
    }

    // ── Pause ──────────────────────────────────────────────────────

    /// Toggle pause state. Clears audio buffer on pause for instant silence.
    pub fn toggle_pause(&mut self, audio_buffer: &Arc<Mutex<Vec<f32>>>) {
        self.paused = !self.paused;
        if self.paused {
            if let Ok(mut buf) = audio_buffer.lock() {
                buf.clear();
            }
        }
        println!("{}", if self.paused { "Paused" } else { "Resumed" });
    }

    /// Record that controller Start was pressed.
    pub fn controller_start_down(&mut self) {
        self.controller_start_held_since = Some(Instant::now());
        self.controller_pause_triggered = false;
    }

    /// Record that controller Start was released.
    pub fn controller_start_up(&mut self) {
        self.controller_start_held_since = None;
        self.controller_pause_triggered = false;
    }

    /// Check if controller Start has been held long enough to trigger pause.
    /// Call once per frame. Returns true if pause was toggled this frame.
    pub fn tick_controller_pause(&mut self, audio_buffer: &Arc<Mutex<Vec<f32>>>) -> bool {
        if let Some(held_since) = self.controller_start_held_since {
            if !self.controller_pause_triggered && held_since.elapsed() >= LONG_PRESS_DURATION {
                self.controller_pause_triggered = true;
                self.toggle_pause(audio_buffer);
                return true;
            }
        }
        false
    }

    // ── Fullscreen ─────────────────────────────────────────────────

    /// Toggle between windowed and fullscreen (Desktop borderless).
    /// Saves the new state to config.toml.
    pub fn toggle_fullscreen(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        app_config: &mut AppConfig,
    ) {
        let window = canvas.window_mut();
        if window.fullscreen_state() == FullscreenType::Off {
            let _ = window.set_fullscreen(FullscreenType::Desktop);
            app_config.fullscreen = true;
            println!("Fullscreen: ON");
        } else {
            let _ = window.set_fullscreen(FullscreenType::Off);
            app_config.fullscreen = false;
            println!("Fullscreen: OFF");
        }
        if let Err(e) = app_config.save() {
            eprintln!("Fullscreen config save error: {}", e);
        }
    }

    /// Apply fullscreen state from config on startup.
    pub fn apply_initial_fullscreen(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        fullscreen: bool,
    ) {
        if fullscreen {
            let _ = canvas.window_mut().set_fullscreen(FullscreenType::Desktop);
        }
    }

    // ── Volume ─────────────────────────────────────────────────────

    /// Adjust volume by delta (clamped to 0..=100, wraps around).
    /// Saves the new value to config.toml.
    pub fn adjust_volume(&mut self, delta: i16, app_config: &mut AppConfig) {
        let v = self.volume as i16 + delta;
        self.volume = if v > 100 {
            0
        } else if v < 0 {
            100
        } else {
            v as u8
        };
        app_config.volume = self.volume;
        self.volume_overlay_until = Some(Instant::now() + VOLUME_DISPLAY_DURATION);
        if let Err(e) = app_config.save() {
            eprintln!("Volume config save error: {}", e);
        }
        println!("Volume: {}%", self.volume);
    }

    /// Returns the current volume scale factor (0.0..=1.0).
    pub fn volume_factor(&self) -> f32 {
        self.volume as f32 / 100.0
    }

    /// Whether the volume overlay should be visible right now.
    pub fn show_volume_overlay(&self) -> bool {
        self.volume_overlay_until
            .is_some_and(|until| Instant::now() < until)
    }

    // ── Save State ─────────────────────────────────────────────────

    /// Save emulator state to ./saves/<name>.state. Creates directory if needed.
    pub fn save_state(&mut self, deck: &mut ControlDeck, rom_name: &str) {
        let path = Self::state_path(rom_name);
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Save dir error: {}", e);
                self.set_state_overlay("SAVE FAILED");
                return;
            }
        }
        match deck.save_state(&path) {
            Ok(()) => {
                println!("State saved: {}", path.display());
                self.set_state_overlay("STATE SAVED");
            }
            Err(e) => {
                eprintln!("Save error: {}", e);
                self.set_state_overlay("SAVE FAILED");
            }
        }
    }

    /// Load emulator state from ./saves/<name>.state. Safe if file missing.
    pub fn load_state(&mut self, deck: &mut ControlDeck, rom_name: &str) {
        let path = Self::state_path(rom_name);
        if !path.exists() {
            self.set_state_overlay("LOAD FAILED");
            return;
        }
        match deck.load_state(&path) {
            Ok(()) => {
                println!("State loaded: {}", path.display());
                self.set_state_overlay("STATE LOADED");
            }
            Err(e) => {
                eprintln!("Load error: {}", e);
                self.set_state_overlay("LOAD FAILED");
            }
        }
    }

    /// Whether the save state overlay should be visible right now.
    pub fn show_state_overlay(&self) -> bool {
        self.state_overlay_until
            .is_some_and(|until| Instant::now() < until)
    }

    /// Get the current state overlay message text.
    pub fn state_overlay_message(&self) -> &str {
        &self.state_overlay_message
    }

    /// Show a state overlay message for STATE_DISPLAY_DURATION.
    fn set_state_overlay(&mut self, msg: &str) {
        self.state_overlay_message = msg.to_string();
        self.state_overlay_until = Some(Instant::now() + STATE_DISPLAY_DURATION);
    }

    /// Compute save file path: ./saves/<rom_name>.state
    fn state_path(rom_name: &str) -> PathBuf {
        let stem = std::path::Path::new(rom_name)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "game".to_string());
        PathBuf::from("saves").join(format!("{}.state", stem))
    }
}
