use crate::config::{ControllerConfig, KeyboardConfig};
use sdl2::controller::{Axis, Button};
use sdl2::keyboard::Keycode;
use tetanes_core::input::JoypadBtnState;
use tetanes_core::prelude::*;

const STICK_THRESHOLD: i16 = 16000;

/// Map an SDL2 Keycode to a canonical name string (case-insensitive matching).
fn keycode_name(key: Keycode) -> &'static str {
    match key {
        Keycode::A => "A",
        Keycode::B => "B",
        Keycode::C => "C",
        Keycode::D => "D",
        Keycode::E => "E",
        Keycode::F => "F",
        Keycode::G => "G",
        Keycode::H => "H",
        Keycode::I => "I",
        Keycode::J => "J",
        Keycode::K => "K",
        Keycode::L => "L",
        Keycode::M => "M",
        Keycode::N => "N",
        Keycode::O => "O",
        Keycode::P => "P",
        Keycode::Q => "Q",
        Keycode::R => "R",
        Keycode::S => "S",
        Keycode::T => "T",
        Keycode::U => "U",
        Keycode::V => "V",
        Keycode::W => "W",
        Keycode::X => "X",
        Keycode::Y => "Y",
        Keycode::Z => "Z",
        Keycode::Num0 => "0",
        Keycode::Num1 => "1",
        Keycode::Num2 => "2",
        Keycode::Num3 => "3",
        Keycode::Num4 => "4",
        Keycode::Num5 => "5",
        Keycode::Num6 => "6",
        Keycode::Num7 => "7",
        Keycode::Num8 => "8",
        Keycode::Num9 => "9",
        Keycode::Return => "RETURN",
        Keycode::Escape => "ESCAPE",
        Keycode::Space => "SPACE",
        Keycode::RShift => "RSHIFT",
        Keycode::LShift => "LSHIFT",
        Keycode::LCtrl => "LCTRL",
        Keycode::RCtrl => "RCTRL",
        Keycode::LAlt => "LALT",
        Keycode::RAlt => "RALT",
        Keycode::Tab => "TAB",
        Keycode::Backspace => "BACKSPACE",
        Keycode::F1 => "F1",
        Keycode::F2 => "F2",
        Keycode::F3 => "F3",
        Keycode::F4 => "F4",
        Keycode::F5 => "F5",
        Keycode::F6 => "F6",
        Keycode::F7 => "F7",
        Keycode::F8 => "F8",
        Keycode::F9 => "F9",
        Keycode::F10 => "F10",
        Keycode::F11 => "F11",
        Keycode::F12 => "F12",
        _ => "",
    }
}

/// Map an SDL2 Button to a canonical name string (case-insensitive matching).
fn button_name(button: Button) -> &'static str {
    match button {
        Button::A => "A",
        Button::B => "B",
        Button::X => "X",
        Button::Y => "Y",
        Button::Back => "Back",
        Button::Guide => "Guide",
        Button::Start => "Start",
        Button::LeftStick => "LeftStick",
        Button::RightStick => "RightStick",
        Button::LeftShoulder => "LeftShoulder",
        Button::RightShoulder => "RightShoulder",
        Button::DPadUp => "DPadUp",
        Button::DPadDown => "DPadDown",
        Button::DPadLeft => "DPadLeft",
        Button::DPadRight => "DPadRight",
        _ => "",
    }
}

/// Match a keyboard key to an NES button using config. Case-insensitive.
fn match_keyboard_nes_button(key: Keycode, config: &KeyboardConfig) -> Option<JoypadBtnState> {
    let name = keycode_name(key);
    if name.is_empty() {
        return None;
    }
    let name_upper = name.to_uppercase();
    if name_upper == config.a.to_uppercase() {
        Some(JoypadBtnState::A)
    } else if name_upper == config.b.to_uppercase() {
        Some(JoypadBtnState::B)
    } else if name_upper == config.start.to_uppercase() {
        Some(JoypadBtnState::START)
    } else if name_upper == config.select.to_uppercase() {
        Some(JoypadBtnState::SELECT)
    } else {
        None
    }
}

/// Match a controller button to an NES button using config. Case-insensitive.
fn match_controller_nes_button(
    button: Button,
    config: &ControllerConfig,
) -> Option<JoypadBtnState> {
    let name = button_name(button);
    if name.is_empty() {
        return None;
    }
    let name_upper = name.to_uppercase();
    if name_upper == config.a.to_uppercase() {
        Some(JoypadBtnState::A)
    } else if name_upper == config.b.to_uppercase() {
        Some(JoypadBtnState::B)
    } else if name_upper == config.start.to_uppercase() {
        Some(JoypadBtnState::START)
    } else if name_upper == config.select.to_uppercase() {
        Some(JoypadBtnState::SELECT)
    } else {
        None
    }
}

/// Handle keyboard key down. Directions hardcoded; A/B/Start/Select config-driven.
pub fn handle_keyboard_down(deck: &mut ControlDeck, key: Keycode, config: &KeyboardConfig) {
    match key {
        Keycode::Up => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::UP, true),
        Keycode::Down => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::DOWN, true),
        Keycode::Left => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::LEFT, true),
        Keycode::Right => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::RIGHT, true),
        other => {
            if let Some(btn) = match_keyboard_nes_button(other, config) {
                deck.joypad_mut(Player::Two).set_button(btn, true);
            }
        }
    }
}

/// Handle keyboard key up. Directions hardcoded; A/B/Start/Select config-driven.
pub fn handle_keyboard_up(deck: &mut ControlDeck, key: Keycode, config: &KeyboardConfig) {
    match key {
        Keycode::Up => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::UP, false),
        Keycode::Down => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::DOWN, false),
        Keycode::Left => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::LEFT, false),
        Keycode::Right => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::RIGHT, false),
        other => {
            if let Some(btn) = match_keyboard_nes_button(other, config) {
                deck.joypad_mut(Player::Two).set_button(btn, false);
            }
        }
    }
}

/// Handle controller button down. DPad hardcoded; A/B/Start/Select config-driven.
pub fn handle_controller_button_down(
    deck: &mut ControlDeck,
    button: Button,
    config: &ControllerConfig,
) {
    match button {
        Button::DPadUp => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::UP, true),
        Button::DPadDown => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::DOWN, true),
        Button::DPadLeft => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::LEFT, true),
        Button::DPadRight => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::RIGHT, true),
        other => {
            if let Some(btn) = match_controller_nes_button(other, config) {
                deck.joypad_mut(Player::One).set_button(btn, true);
            }
        }
    }
}

/// Handle controller button up. DPad hardcoded; A/B/Start/Select config-driven.
pub fn handle_controller_button_up(
    deck: &mut ControlDeck,
    button: Button,
    config: &ControllerConfig,
) {
    match button {
        Button::DPadUp => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::UP, false),
        Button::DPadDown => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::DOWN, false),
        Button::DPadLeft => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::LEFT, false),
        Button::DPadRight => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::RIGHT, false),
        other => {
            if let Some(btn) = match_controller_nes_button(other, config) {
                deck.joypad_mut(Player::One).set_button(btn, false);
            }
        }
    }
}

/// Handle controller axis motion (Left Stick). Not configurable.
pub fn handle_controller_axis(deck: &mut ControlDeck, axis: Axis, value: i16) {
    match axis {
        Axis::LeftX => {
            if value > STICK_THRESHOLD {
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::RIGHT, true);
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::LEFT, false);
            } else if value < -STICK_THRESHOLD {
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::LEFT, true);
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::RIGHT, false);
            } else {
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::LEFT, false);
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::RIGHT, false);
            }
        }
        Axis::LeftY => {
            if value > STICK_THRESHOLD {
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::DOWN, true);
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::UP, false);
            } else if value < -STICK_THRESHOLD {
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::UP, true);
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::DOWN, false);
            } else {
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::UP, false);
                deck.joypad_mut(Player::One)
                    .set_button(JoypadBtnState::DOWN, false);
            }
        }
        _ => {}
    }
}
