use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tetanes_core::prelude::*;

mod input;
mod overlay;

// Audio callback structure
struct NesAudioCallback {
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCallback for NesAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut buffer = self.buffer.lock().unwrap();
        let len = out.len().min(buffer.len());

        // Copy samples to output
        for i in 0..len {
            out[i] = buffer[i];
        }

        // Fill remaining with silence
        for i in len..out.len() {
            out[i] = 0.0;
        }

        // Remove played samples
        buffer.drain(..len);
    }
}

fn find_bundle_resource(name: &str) -> Option<String> {
    // Try to find resource relative to the executable
    let exe = env::current_exe().ok()?;
    let resources = exe.parent()?.parent()?.join("Resources").join(name);
    if resources.exists() {
        Some(resources.to_string_lossy().to_string())
    } else {
        None
    }
}

fn main() -> Result<(), String> {
    println!("=== TurtleBox ===\n");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Determine ROM path
    let rom_path = if args.len() > 1 {
        let path = &args[1];
        println!("ROM path from argument: {}", path);
        path.clone()
    } else {
        // Try bundled TMNT3.nes first
        if let Some(bundled) = find_bundle_resource("TMNT3.nes") {
            println!("Using bundled ROM: {}", bundled);
            bundled
        } else if let Some(bundled) = find_bundle_resource("spritecans.nes") {
            println!("Using bundled test ROM: {}", bundled);
            bundled
        } else {
            // Fall back to file dialog
            println!("No ROM found, opening file dialog...");
            let file = rfd::FileDialog::new()
                .add_filter("NES ROM", &["nes"])
                .set_title("Select NES ROM")
                .pick_file();
            match file {
                Some(path) => path.to_string_lossy().to_string(),
                None => return Err("No ROM selected".to_string()),
            }
        }
    };

    // Validate ROM file exists
    if !Path::new(&rom_path).exists() {
        return Err(format!("ROM file not found: {}", rom_path));
    }

    // Validate ROM file extension
    if !rom_path.to_lowercase().ends_with(".nes") {
        return Err(format!(
            "Invalid ROM file extension: {}. Expected .nes",
            rom_path
        ));
    }

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    let game_controller_subsystem = sdl_context.game_controller()?;

    // Get ROM filename for window title
    let rom_filename = Path::new(&rom_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let window_title = format!("TurtleBox - {}", rom_filename);

    let window = video_subsystem
        .window(&window_title, 960, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    // Create texture for NES output (256x240 RGBA)
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, 256, 240)
        .map_err(|e| e.to_string())?;

    // Initialize SDL2 Audio
    let audio_buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let audio_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: Some(1024),
    };

    let audio_device = audio_subsystem
        .open_playback(None, &audio_spec, |spec| {
            println!(
                "  ✓ Audio device opened: {} Hz, {} channels, {} samples",
                spec.freq, spec.channels, spec.samples
            );
            NesAudioCallback {
                buffer: audio_buffer.clone(),
            }
        })
        .map_err(|e| format!("Failed to open audio: {}", e))?;

    // Start audio playback
    audio_device.resume();
    println!("  ✓ Audio playback started");

    // Initialize tetanes-core
    println!("\nStep 1: Creating ControlDeck...");
    let mut deck = ControlDeck::with_config(Config {
        region: NesRegion::Ntsc,
        ..Default::default()
    });
    println!("  ✓ ControlDeck created");

    // Load ROM
    println!("\nStep 2: Loading ROM...");
    match deck.load_rom_path(&rom_path) {
        Ok(loaded_rom) => {
            println!("  ✓ ROM loaded: {}", loaded_rom.name);
            println!("  Region: {:?}", loaded_rom.region);
            println!("  Battery: {}", loaded_rom.battery_backed);
        }
        Err(e) => {
            return Err(format!("Failed to load ROM: {}", e));
        }
    }

    // Check for already connected controllers
    let mut controller: Option<sdl2::controller::GameController> = None;
    for id in 0..game_controller_subsystem.num_joysticks().unwrap_or(0) {
        if game_controller_subsystem.is_game_controller(id) {
            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    println!("\n  ✓ Controller connected: {}", c.name());
                    controller = Some(c);
                    break;
                }
                Err(_) => {}
            }
        }
    }

    // Main loop
    println!("\nStep 3: Starting render loop...");
    println!("  P1 Xbox Controller: DPad/Left Stick=Move, LB=Jump, RB=Attack");
    println!("  P2 Keyboard: Arrows=Move, A=Jump, D=Attack, Enter=Start, RShift=Select");
    println!("  Press ESC to quit\n");

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_count: u32 = 0;
    let mut show_help = false;
    let frame_duration = Duration::from_micros(16_639); // ~60.0988 fps (NTSC)

    'running: loop {
        let frame_start = Instant::now();
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    show_help = !show_help;
                }

                // Keyboard input - Key Down (Player 2)
                Event::KeyDown {
                    keycode: Some(key), ..
                } if !show_help => {
                    input::handle_keyboard_down(&mut deck, key);
                }

                // Keyboard input - Key Up (Player 2)
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    input::handle_keyboard_up(&mut deck, key);
                }

                // Controller connected
                Event::ControllerDeviceAdded { which, .. } => {
                    if controller.is_none() {
                        match game_controller_subsystem.open(which) {
                            Ok(c) => {
                                println!("  ✓ Controller connected: {}", c.name());
                                controller = Some(c);
                            }
                            Err(e) => println!("  ✗ Controller error: {}", e),
                        }
                    }
                }

                // Controller disconnected
                Event::ControllerDeviceRemoved { .. } => {
                    controller = None;
                    println!("  ✗ Controller disconnected");
                }

                // Controller button down
                Event::ControllerButtonDown { button, .. } => {
                    input::handle_controller_button_down(&mut deck, button);
                }

                // Controller button up
                Event::ControllerButtonUp { button, .. } => {
                    input::handle_controller_button_up(&mut deck, button);
                }

                // Left Stick axis motion
                Event::ControllerAxisMotion { axis, value, .. } => {
                    input::handle_controller_axis(&mut deck, axis, value);
                }

                _ => {}
            }
        }

        // Emulate one frame
        match deck.clock_frame() {
            Ok(()) => frame_count += 1,
            Err(e) => {
                println!("Frame {} error: {}", frame_count + 1, e);
                break 'running;
            }
        }

        // Get audio samples and add to buffer
        let audio_samples = deck.audio_samples();
        if !audio_samples.is_empty() {
            let mut buffer = audio_buffer.lock().unwrap();
            buffer.extend_from_slice(audio_samples);
        }
        deck.clear_audio_samples();

        // Get frame buffer (RGBA 256x240)
        let frame_buffer = deck.frame_buffer();

        // Update texture
        texture
            .update(None, frame_buffer, 256 * 4)
            .map_err(|e| e.to_string())?;

        // Render
        canvas.clear();
        canvas
            .copy(&texture, None, None)
            .map_err(|e| e.to_string())?;

        if show_help {
            overlay::render_help_overlay(&mut canvas);
        }

        canvas.present();

        // Frame rate limiting (NTSC ~60fps)
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    println!("\n=== Summary ===");
    println!("ROM: {}", rom_filename);
    println!("Total frames rendered: {}", frame_count);

    Ok(())
}
