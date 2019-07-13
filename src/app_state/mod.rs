use crate::scale::Scale;
use std::collections::HashMap;

pub enum PlayMode {
    Single,
    Chord,
}

pub struct AppState {
    pub scale: Box<dyn Scale>,
    pub play_mode: PlayMode,
    pub pressed_keys: HashMap<String, bool>,
}

impl AppState {
    pub fn new(scale: Box<dyn Scale>) -> Self {
        return AppState {
            scale,
            play_mode: PlayMode::Single,
            pressed_keys: HashMap::new(),
        };
    }

    pub fn set_scale(&mut self, scale: Box<dyn Scale>) {
        self.scale = scale;
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

    pub fn play_mode_label(&self) -> String {
        return match self.play_mode {
            PlayMode::Single => "Single Note".into(),
            PlayMode::Chord => "Chord".into(),
        };
    }
}
