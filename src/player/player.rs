use crate::scale::Chord;
use crate::scale::NaturalMinor;
use crate::events::{Event, NoteMessage};
use crate::player::Metronome;
use crossbeam_channel::{tick, Receiver, Sender};
use log::error;
use rand::Rng;
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
    running: bool,
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
            running: true,
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

    pub fn playing(&self) -> bool {
        return self.running;
    }

    pub fn toggle_playing(&mut self) {
        self.running = !self.running;
    }
}

impl Player {
    pub fn new(
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
                        if state.playing() {
                            let notes = state.next();

                            for n in notes {
                                if play_chord {
                                    let notes = chord.get_notes(n);

                                    for pn in notes {
                                        play_note(pn);
                                    }
                                } else {
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
                            Event::Pause => {
                                state.toggle_playing();
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
