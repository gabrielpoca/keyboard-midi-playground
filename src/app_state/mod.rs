use crate::scale::Scale;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub enum PlayMode {
    Single,
    Chord,
}

pub struct AppState {
    input_mode: u8,
    pub scale: Box<dyn Scale>,
    pub play_mode: PlayMode,
    pub pressed_keys: HashMap<String, bool>,
}

impl AppState {
    pub fn new(scale: Box<dyn Scale>) -> Self {
        return AppState {
            input_mode: 0,
            scale,
            play_mode: PlayMode::Single,
            pressed_keys: HashMap::new(),
        };
    }

    pub fn set_scale(&mut self, scale: Box<dyn Scale>) {
        self.scale = scale;
    }

    pub fn toggle_input_mode(&mut self) {
        if self.input_mode == 0 {
            self.input_mode = 1
        } else {
            self.input_mode = 0
        }
    }

    pub fn toggle_play_mode(&mut self) {
        match self.play_mode {
            PlayMode::Single => self.play_mode = PlayMode::Chord,
            PlayMode::Chord => self.play_mode = PlayMode::Single,
        }
    }

    pub fn play_chord(&self) -> bool {
        return match self.play_mode {
            PlayMode::Single => false,
            PlayMode::Chord => true,
        };
    }
}
