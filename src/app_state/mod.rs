use crate::events::*;
use crate::scale::Scale;
use std::collections::HashMap;

pub enum PlayMode {
    Single,
    Chord,
}

pub struct AppState {
    pub scale: Box<dyn Scale + Sync + Send>,
    pub play_mode: PlayMode,
    pub pressed_keys: HashMap<Key, bool>,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl AppState {
    pub fn new(scale: Box<dyn Scale + Sync + Send>) -> Self {
        return AppState {
            scale,
            play_mode: PlayMode::Single,
            pressed_keys: HashMap::new(),
            screen_width: 800,
            screen_height: 600,
        };
    }

    pub fn set_scale(&mut self, scale: Box<dyn Scale + Sync + Send>) {
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
