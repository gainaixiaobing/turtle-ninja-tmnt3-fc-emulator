use sdl2::controller::{Axis, Button};
use sdl2::keyboard::Keycode;
use tetanes_core::input::JoypadBtnState;
use tetanes_core::prelude::*;

const STICK_THRESHOLD: i16 = 16000;

pub fn handle_keyboard_down(deck: &mut ControlDeck, key: Keycode) {
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
        Keycode::A => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::A, true),
        Keycode::D => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::B, true),
        Keycode::Return => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::START, true),
        Keycode::RShift => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::SELECT, true),
        _ => {}
    }
}

pub fn handle_keyboard_up(deck: &mut ControlDeck, key: Keycode) {
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
        Keycode::A => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::A, false),
        Keycode::D => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::B, false),
        Keycode::Return => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::START, false),
        Keycode::RShift => deck
            .joypad_mut(Player::Two)
            .set_button(JoypadBtnState::SELECT, false),
        _ => {}
    }
}

pub fn handle_controller_button_down(deck: &mut ControlDeck, button: Button) {
    match button {
        // LB -> NES A (jump)
        Button::LeftShoulder => {
            deck.joypad_mut(Player::One)
                .set_button(JoypadBtnState::A, true);
        }
        // RB -> NES B (attack)
        Button::RightShoulder => {
            deck.joypad_mut(Player::One)
                .set_button(JoypadBtnState::B, true);
        }
        // DPad
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
        // Start/Select
        Button::Start => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::START, true),
        Button::Back => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::SELECT, true),
        _ => {}
    }
}

pub fn handle_controller_button_up(deck: &mut ControlDeck, button: Button) {
    match button {
        // LB -> NES A (jump)
        Button::LeftShoulder => {
            deck.joypad_mut(Player::One)
                .set_button(JoypadBtnState::A, false);
        }
        // RB -> NES B (attack)
        Button::RightShoulder => {
            deck.joypad_mut(Player::One)
                .set_button(JoypadBtnState::B, false);
        }
        // DPad
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
        // Start/Select
        Button::Start => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::START, false),
        Button::Back => deck
            .joypad_mut(Player::One)
            .set_button(JoypadBtnState::SELECT, false),
        _ => {}
    }
}

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
            // SDL2 Y axis: negative = up, positive = down
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
