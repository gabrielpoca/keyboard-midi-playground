use crate::app_state::*;
use crate::events::*;
use crate::scale::*;
use crossbeam_channel::Sender;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::*;

static NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub struct KeyboardHandler {
    pub emitter: Sender<Event>,
    pub app_state: Arc<RwLock<AppState>>,
    pub mappings: HashMap<u32, Box<str>>,
}

impl KeyboardHandler {
    pub fn new(emitter: Sender<Event>, app_state: Arc<RwLock<AppState>>) -> Self {
        let mut index = 9;
        let mut mappings = HashMap::new();

        for note in 21..108 {
            mappings.insert(note as u32, NOTES[index].into());
            if index == 11 {
                index = 0;
            } else {
                index += 1;
            }
        }

        return KeyboardHandler {
            emitter,
            app_state,
            mappings,
        };
    }

    pub fn reset(&self) {
        let app_state = self.app_state.read().unwrap();

        for (key, val) in app_state.pressed_keys.iter() {
            if *val == true {
                let notes: Vec<u32> = self.key_to_midi_notes(key.clone());

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

    pub fn key_to_note<'a>(&self, key: Key) -> Cow<str> {
        let label = format!("{:?}", key);
        let midi_note = self.key_to_midi(key);

        if midi_note == None {
            return Cow::from(label);
        }

        return match self.mappings.get(&midi_note.unwrap()) {
            Some(note) => Cow::from(note.to_string()),
            None => Cow::from(label),
        };
    }

    pub fn handle_key_on(&self, key: Key) {
        let notes: Vec<u32> = self.key_to_midi_notes(key);

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
        let notes: Vec<u32> = self.key_to_midi_notes(key);

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

    fn key_to_midi_notes(&self, key: Key) -> Vec<u32> {
        let app_state = self.app_state.read().unwrap();
        let note = self.key_to_midi(key);

        match note {
            Some(note) => {
                if app_state.play_chord() {
                    let scale = &app_state.scale;
                    return chord::get(scale, note);
                } else {
                    return [note].to_vec();
                }
            }
            None => {
                return Vec::new();
            }
        };
    }

    fn key_to_midi(&self, key: Key) -> Option<u32> {
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
            _ => -1,
        };

        if index == -1 {
            return None;
        }

        let app_state = self.app_state.read().unwrap();
        let scale = &app_state.scale;

        return Some(scale.note(index));
    }
}
