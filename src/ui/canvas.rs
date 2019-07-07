use super::Render;
use crate::events;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

struct InputState {
    mode: u8,
    mappings: HashMap<char, u32>,
}

impl InputState {
    fn new() -> InputState {
        let mut mappings = HashMap::new();
        mappings.insert('a', 0);
        mappings.insert('s', 1);
        mappings.insert('d', 2);
        mappings.insert('f', 3);
        mappings.insert('g', 4);
        mappings.insert('h', 5);
        mappings.insert('j', 6);
        mappings.insert('k', 7);
        mappings.insert('l', 8);
        mappings.insert(';', 9);

        return InputState { mode: 0, mappings };
    }

    fn edit_mode(&mut self) {
        self.mode = 1;
    }

    fn play_mode(&mut self) {
        self.mode = 0;
    }

    fn is_play_mode(&self) -> bool {
        return self.mode == 0;
    }

    fn transform_key(&self, key: char) -> u32 {
        if self.mappings.contains_key(&key.into()) {
            match self.mappings.get(&key.into()) {
                Some(&n) => return n,
                None => return 0,
            }
        } else {
            return 0;
        }
    }
}
