use crate::app_state::*;
use crate::events::*;
use crate::scale::*;
use crossbeam_channel::Sender;
use log::info;
use std::sync::*;

pub struct KeyboardToMidi {
    pub emitter: Sender<Event>,
    pub app_state: Arc<RwLock<AppState>>,
}

impl KeyboardToMidi {
    pub fn reset(&self) {
        let app_state = self.app_state.read().unwrap();

        for (key, val) in app_state.pressed_keys.iter() {
            if *val == true {
                let notes: Vec<u32> = self.get_notes(key.clone());

                for note in notes.iter() {
                    self.emitter
                        .send(Event::Note {
                            message: NoteMessage::Off,
                            note: *note as u8,
                            velocity: 10,
                        })
                        .unwrap_or_default();
                }
            }
        }
    }

    pub fn handle_key_on(&self, key: Key) {
        info!("on {}", key);
        let notes: Vec<u32> = self.get_notes(key);

        for note in notes.iter() {
            self.emitter
                .send(Event::Note {
                    message: NoteMessage::On,
                    note: *note as u8,
                    velocity: 10,
                })
                .unwrap_or_default();
        }
    }

    pub fn handle_key_off(&self, key: Key) {
        info!("off {}", key);
        let notes: Vec<u32> = self.get_notes(key);

        for note in notes.iter() {
            self.emitter
                .send(Event::Note {
                    message: NoteMessage::Off,
                    note: *note as u8,
                    velocity: 10,
                })
                .unwrap_or_default();
        }
    }

    fn get_notes(&self, key: Key) -> Vec<u32> {
        let app_state = self.app_state.read().unwrap();
        let note = self.keycode_to_midi(key);
        let mut notes: Vec<u32> = [note].to_vec();

        if app_state.play_chord() {
            let scale = &app_state.scale;
            notes = chord::get(scale, note);
        }

        return notes;
    }

    fn keycode_to_midi(&self, key: Key) -> u32 {
        let index = match key {
            Key::W => 4,
            Key::E => 5,
            Key::R => 6,
            Key::T => 7,
            Key::Y => 8,
            Key::U => 9,
            Key::I => 10,
            Key::O => 11,
            Key::A => 0,
            Key::S => 1,
            Key::D => 2,
            Key::F => 3,
            Key::G => 4,
            Key::H => 5,
            Key::J => 6,
            Key::K => 7,
            Key::L => 8,
            _ => 0,
        };

        let app_state = self.app_state.read().unwrap();
        let scale = &app_state.scale;

        return scale.note(index);
    }
}
