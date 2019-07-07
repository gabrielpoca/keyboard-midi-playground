use crate::app_state::*;
use crate::events::{Event, NoteMessage};
use crate::scale::Chord;
use crate::scale::NaturalMinor;
use crossbeam_channel::{Receiver, Sender};
use rand::Rng;
use std::sync::*;
use std::thread;

pub struct Player {
    handle: thread::JoinHandle<()>,
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerNote {
    pub note: u32,
    pub message: NoteMessage,
}

struct PlayerState {
    current: usize,
    notes: Vec<Vec<PlayerNote>>,
}

impl PlayerState {
    pub fn new(my_notes: Vec<u32>) -> PlayerState {
        let mut notes = Vec::new();

        for note in my_notes.iter() {
            notes.push(vec![PlayerNote {
                note: *note,
                message: NoteMessage::On,
            }]);

            notes.push(vec![]);
            notes.push(vec![]);

            notes.push(vec![PlayerNote {
                note: *note,
                message: NoteMessage::Off,
            }]);
        }

        return PlayerState {
            current: 0,
            notes: notes,
        };
    }

    pub fn next(&mut self) -> Vec<PlayerNote> {
        let note = self.notes[self.current].clone();

        if self.current == self.notes.len() - 1 {
            self.current = 0;
        } else {
            self.current += 1;
        }

        return note;
    }
}

impl Player {
    pub fn new(
        app_state: Arc<RwLock<AppState>>,
        tick: Receiver<Event>,
        events_emitter: Sender<Event>,
        events_recv: Receiver<Event>,
        play_chord: bool,
        notes: Vec<u32>,
        scale: NaturalMinor,
    ) -> Player {
        let scale = scale.clone();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut state = PlayerState::new(notes);
            let chord = Chord::new(scale);

            let mut play_note = |player_note: PlayerNote| {
                let velocity = rng.gen_range(10, 100);

                let _ = events_emitter
                    .send(Event::Note {
                        message: player_note.message,
                        note: player_note.note as u8,
                        velocity,
                    })
                    .unwrap();
            };

            loop {
                select! {
                    recv(tick) -> _ => {
                        if app_state.read().unwrap().playing() {
                            let notes = state.next();

                            for n in notes {
                                if play_chord {
                                    let notes = chord.get_notes(n);

                                    for pn in notes {
                                        play_note(pn);
                                    }
                                } else {
                                    println!("ASDASD {:?}", n);
                                    play_note(n);
                                }
                            }
                        }
                    }
                    recv(events_recv) -> msg => {
                        match msg.unwrap() {
                            Event::Quit => {
                                drop(tick);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        return Player { handle };
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
