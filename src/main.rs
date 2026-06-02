use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::controller::{Button, Axis};
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tetanes_core::prelude::*;
use tetanes_core::input::JoypadBtnState;

const STICK_THRESHOLD: i16 = 16000;

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
        let test_rom = "spritecans.nes";
        println!("No ROM specified, using test ROM: {}", test_rom);
        test_rom.to_string()
    };

    // Validate ROM file exists
    if !Path::new(&rom_path).exists() {
        return Err(format!("ROM file not found: {}", rom_path));
    }

    // Validate ROM file extension
    if !rom_path.to_lowercase().ends_with(".nes") {
        return Err(format!("Invalid ROM file extension: {}. Expected .nes", rom_path));
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

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

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

    let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| {
        println!("  ✓ Audio device opened: {} Hz, {} channels, {} samples",
            spec.freq, spec.channels, spec.samples);
        NesAudioCallback {
            buffer: audio_buffer.clone(),
        }
    }).map_err(|e| format!("Failed to open audio: {}", e))?;

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

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                // Keyboard input - Key Down (Player 2)
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Up => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::UP, true),
                        Keycode::Down => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::DOWN, true),
                        Keycode::Left => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::LEFT, true),
                        Keycode::Right => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::RIGHT, true),
                        Keycode::A => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::A, true),
                        Keycode::D => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::B, true),
                        Keycode::Return => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::START, true),
                        Keycode::RShift => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::SELECT, true),
                        _ => {}
                    }
                }

                // Keyboard input - Key Up (Player 2)
                Event::KeyUp { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Up => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::UP, false),
                        Keycode::Down => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::DOWN, false),
                        Keycode::Left => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::LEFT, false),
                        Keycode::Right => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::RIGHT, false),
                        Keycode::A => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::A, false),
                        Keycode::D => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::B, false),
                        Keycode::Return => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::START, false),
                        Keycode::RShift => deck.joypad_mut(Player::Two).set_button(JoypadBtnState::SELECT, false),
                        _ => {}
                    }
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
                    match button {
                        // LB → NES A (jump)
                        Button::LeftShoulder => {
                            deck.joypad_mut(Player::One).set_button(JoypadBtnState::A, true);
                        }
                        // RB → NES B (attack)
                        Button::RightShoulder => {
                            deck.joypad_mut(Player::One).set_button(JoypadBtnState::B, true);
                        }
                        // DPad
                        Button::DPadUp => deck.joypad_mut(Player::One).set_button(JoypadBtnState::UP, true),
                        Button::DPadDown => deck.joypad_mut(Player::One).set_button(JoypadBtnState::DOWN, true),
                        Button::DPadLeft => deck.joypad_mut(Player::One).set_button(JoypadBtnState::LEFT, true),
                        Button::DPadRight => deck.joypad_mut(Player::One).set_button(JoypadBtnState::RIGHT, true),
                        // Start/Select
                        Button::Start => deck.joypad_mut(Player::One).set_button(JoypadBtnState::START, true),
                        Button::Back => deck.joypad_mut(Player::One).set_button(JoypadBtnState::SELECT, true),
                        _ => {}
                    }
                }

                // Controller button up
                Event::ControllerButtonUp { button, .. } => {
                    match button {
                        // LB → NES A (jump)
                        Button::LeftShoulder => {
                            deck.joypad_mut(Player::One).set_button(JoypadBtnState::A, false);
                        }
                        // RB → NES B (attack)
                        Button::RightShoulder => {
                            deck.joypad_mut(Player::One).set_button(JoypadBtnState::B, false);
                        }
                        // DPad
                        Button::DPadUp => deck.joypad_mut(Player::One).set_button(JoypadBtnState::UP, false),
                        Button::DPadDown => deck.joypad_mut(Player::One).set_button(JoypadBtnState::DOWN, false),
                        Button::DPadLeft => deck.joypad_mut(Player::One).set_button(JoypadBtnState::LEFT, false),
                        Button::DPadRight => deck.joypad_mut(Player::One).set_button(JoypadBtnState::RIGHT, false),
                        // Start/Select
                        Button::Start => deck.joypad_mut(Player::One).set_button(JoypadBtnState::START, false),
                        Button::Back => deck.joypad_mut(Player::One).set_button(JoypadBtnState::SELECT, false),
                        _ => {}
                    }
                }

                // Left Stick axis motion
                Event::ControllerAxisMotion { axis, value, .. } => {
                    match axis {
                        Axis::LeftX => {
                            if value > STICK_THRESHOLD {
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::RIGHT, true);
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::LEFT, false);
                            } else if value < -STICK_THRESHOLD {
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::LEFT, true);
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::RIGHT, false);
                            } else {
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::LEFT, false);
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::RIGHT, false);
                            }
                        }
                        Axis::LeftY => {
                            // SDL2 Y axis: negative = up, positive = down
                            if value > STICK_THRESHOLD {
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::DOWN, true);
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::UP, false);
                            } else if value < -STICK_THRESHOLD {
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::UP, true);
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::DOWN, false);
                            } else {
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::UP, false);
                                deck.joypad_mut(Player::One).set_button(JoypadBtnState::DOWN, false);
                            }
                        }
                        _ => {}
                    }
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
        texture.update(None, frame_buffer, 256 * 4)
            .map_err(|e| e.to_string())?;

        // Render
        canvas.clear();
        canvas.copy(&texture, None, None)
            .map_err(|e| e.to_string())?;
        canvas.present();
    }

    println!("\n=== Summary ===");
    println!("ROM: {}", rom_filename);
    println!("Total frames rendered: {}", frame_count);

    Ok(())
}
