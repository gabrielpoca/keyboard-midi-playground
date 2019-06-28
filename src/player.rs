use crossbeam_channel::tick;
use rand::Rng;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::events::{Event, NoteMessage};

pub struct Player {
    handle: thread::JoinHandle<()>,
}
impl Player {
    pub fn new(
        events_emitter: crossbeam_channel::Sender<Event>,
        events_recv: crossbeam_channel::Receiver<Event>,
    ) -> Player {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let ticker = tick(Duration::from_millis(1000));

            let mut play_note = |note: u8, duration: u64| {
                let velocity = rng.gen_range(10, 100);

                let _ = events_emitter
                    .send(Event::Note {
                        message: NoteMessage::On,
                        note,
                        velocity,
                    })
                    .unwrap();

                sleep(Duration::from_millis(duration));

                let _ = events_emitter
                    .send(Event::Note {
                        message: NoteMessage::Off,
                        note,
                        velocity,
                    })
                    .unwrap();
            };

            let mut index = 0;
            let mut running = true;
            let root = 60;
            let melody = vec![
                root,
                root + 2,
                root + 3,
                root + 5,
                root + 7,
                root + 8,
                root + 10,
                root + 12,
            ];

            loop {
                select! {
                    recv(ticker) -> _ => {
                        if running {
                            play_note(melody[index], 500);

                            if index == melody.len() - 1 {
                                index = 0;
                            } else {
                                index += 1;
                            }
                        }
                    }
                    recv(events_recv) -> msg => {
                        match msg.unwrap() {
                            Event::Quit { } => {
                                drop(tick);
                                break;
                            }
                            Event::Pause {} => {
                                running = !running;
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
