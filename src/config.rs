use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "config.toml";

/// Top-level application config.
/// Named `AppConfig` to avoid collision with `tetanes_core::Config`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub fullscreen: bool,
    pub volume: u8,
    #[serde(default)]
    pub keyboard: KeyboardConfig,
    #[serde(default)]
    pub controller: ControllerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardConfig {
    /// Keyboard key name mapped to NES A
    pub a: String,
    /// Keyboard key name mapped to NES B
    pub b: String,
    /// Keyboard key name mapped to NES Start
    pub start: String,
    /// Keyboard key name mapped to NES Select
    pub select: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerConfig {
    /// SDL2 controller button name mapped to NES A
    pub a: String,
    /// SDL2 controller button name mapped to NES B
    pub b: String,
    /// SDL2 controller button name mapped to NES Start
    pub start: String,
    /// SDL2 controller button name mapped to NES Select
    pub select: String,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            a: "A".to_string(),
            b: "D".to_string(),
            start: "RETURN".to_string(),
            select: "RSHIFT".to_string(),
        }
    }
}

impl Default for ControllerConfig {
    fn default() -> Self {
        Self {
            a: "LeftShoulder".to_string(),
            b: "RightShoulder".to_string(),
            start: "Start".to_string(),
            select: "Back".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            fullscreen: false,
            volume: 100,
            keyboard: KeyboardConfig::default(),
            controller: ControllerConfig::default(),
        }
    }
}

impl AppConfig {
    /// Load config from `config.toml`. If missing or invalid, generate defaults.
    pub fn load() -> Self {
        let path = Self::config_path();

        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<AppConfig>(&content) {
                    Ok(config) => {
                        println!("  Config loaded from: {}", path.display());
                        return config;
                    }
                    Err(e) => {
                        eprintln!("  Config parse error ({}), using defaults", e);
                    }
                },
                Err(e) => {
                    eprintln!("  Config read error ({}), using defaults", e);
                }
            }
        }

        // No valid config found — create and save defaults
        let config = AppConfig::default();
        if let Err(e) = config.save() {
            eprintln!("  Config save error: {}", e);
        }
        config
    }

    /// Serialize config to TOML and write to disk.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        let toml_string = toml::to_string_pretty(self).map_err(|e| format!("serialize: {}", e))?;
        std::fs::write(&path, toml_string).map_err(|e| format!("write: {}", e))?;
        println!("  Default config saved to: {}", path.display());
        Ok(())
    }

    /// Path to `config.toml` — next to the executable.
    fn config_path() -> PathBuf {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                return dir.join(CONFIG_FILENAME);
            }
        }
        PathBuf::from(CONFIG_FILENAME)
    }
}
