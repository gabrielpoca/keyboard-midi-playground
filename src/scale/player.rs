use super::chord::Chord;
use super::harmonic_minor::HarmonicMinor;
use super::natural_minor::NaturalMinor;
use super::scale::Scale;
use crate::events::{Event, NoteMessage};
use crossbeam_channel::tick;
use log::error;
use rand::Rng;
use std::thread;
use std::time::Duration;

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
    pub fn new<S: Scale>(scale: S) -> PlayerState {
        let mut notes: Vec<Vec<PlayerNote>> = Vec::new();
        let my_scale = [
            scale.note(-3),
            scale.note(0),
            scale.note(-3),
            scale.note(0),
            scale.note(4),
            scale.note(2),
            scale.note(3),
            scale.note(1),
        ];

        for note in my_scale.iter() {
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
        events_emitter: crossbeam_channel::Sender<Event>,
        events_recv: crossbeam_channel::Receiver<Event>,
    ) -> Player {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let ticker = tick(Duration::from_millis(200));

            let mut state = PlayerState::new(HarmonicMinor::new(60));
            let chord = Chord::new(HarmonicMinor::new(60));

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
                    recv(ticker) -> _ => {
                        if state.playing() {
                            let notes = state.next();

                            for n in notes {
                                let notes = chord.get_notes(n);

                                for pn in notes {
                                    play_note(pn);
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
