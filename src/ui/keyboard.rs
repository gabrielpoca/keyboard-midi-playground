use crate::app_state::*;
use crate::events;
use crate::scale::*;
use crossbeam_channel::Sender;
use log::info;
use sdl2::keyboard::Keycode;
use std::sync::*;

pub struct KeyboardToMidi {
    pub emitter: Sender<events::Event>,
    pub app_state: Arc<RwLock<AppState>>,
}

impl KeyboardToMidi {
    pub fn handle_key_on(&self, keycode: Keycode) {
        info!("on {}", keycode);
        let app_state = self.app_state.read().unwrap();
        let note = self.keycode_to_midi(keycode);
        let mut notes: Vec<u32> = [note].to_vec();

        if app_state.play_chord() {
            let scale = &app_state.scale;
            notes = chord::get(scale, note);
        }

        for note in notes.iter() {
            self.emitter
                .send(events::Event::Note {
                    message: events::NoteMessage::On,
                    note: *note as u8,
                    velocity: 10,
                })
                .unwrap();
        }
    }

    pub fn handle_key_off(&self, keycode: Keycode) {
        info!("off {}", keycode);
        let app_state = self.app_state.read().unwrap();
        let note = self.keycode_to_midi(keycode);
        let mut notes: Vec<u32> = [note].to_vec();

        if app_state.play_chord() {
            let scale = &app_state.scale;
            notes = chord::get(scale, note);
        }

        for note in notes.iter() {
            self.emitter
                .send(events::Event::Note {
                    message: events::NoteMessage::Off,
                    note: *note as u8,
                    velocity: 10,
                })
                .unwrap();
        }
    }

    pub fn keycode_to_midi(&self, keycode: Keycode) -> u32 {
        let index = match keycode {
            Keycode::W => 4,
            Keycode::E => 5,
            Keycode::R => 6,
            Keycode::T => 7,
            Keycode::Y => 8,
            Keycode::U => 9,
            Keycode::I => 10,
            Keycode::O => 11,
            Keycode::A => 0,
            Keycode::S => 1,
            Keycode::D => 2,
            Keycode::F => 3,
            Keycode::G => 4,
            Keycode::H => 5,
            Keycode::J => 6,
            Keycode::K => 7,
            Keycode::L => 8,
            _ => 0,
        };

        let app_state = self.app_state.read().unwrap();
        let scale = &app_state.scale;

        return scale.note(index);
    }
}
