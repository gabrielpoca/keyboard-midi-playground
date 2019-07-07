pub struct AppState {
    input_mode: u8,
    playing: bool,
}

impl AppState {
    pub fn new() -> AppState {
        return AppState {
            playing: false,
            input_mode: 0,
        };
    }

    pub fn playing(&self) -> bool {
        return self.playing;
    }

    pub fn playing_label(&self) -> String {
        if self.playing() {
            return "Playing".into();
        } else {
            return "Paused".into();
        }
    }

    pub fn toggle_play(&mut self) {
        self.playing = !self.playing;
    }

    pub fn toggle_input_mode(&mut self) {
        if self.input_mode == 0 {
            self.input_mode = 1
        } else {
            self.input_mode = 0
        }
    }
}
