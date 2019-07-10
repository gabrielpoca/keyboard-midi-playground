use crate::scale::Scale;

pub struct AppState {
    input_mode: u8,
    pub scale: Box<dyn Scale>,
}

impl AppState {
    pub fn new(scale: Box<dyn Scale>) -> Self {
        return AppState {
            input_mode: 0,
            scale,
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
}
