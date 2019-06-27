use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::events::{Event, NoteMessage};

pub struct Player {
    handle: thread::JoinHandle<()>,
}

impl Player {
    pub fn new(conn: Arc<Mutex<(u8)>>, events_emitter: crossbeam_channel::Sender<Event>) -> Player {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

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

            for _i in 0..3 {
                let num = conn.lock().unwrap();
                let base = *num;
                let root = 60;
                std::mem::drop(num);
                play_note(root + base, 500);
                play_note(root + 2 + base, 500);
                play_note(root + 3 + base, 500);
                play_note(root + 5 + base, 500);
                play_note(root + 7 + base, 500);
                play_note(root + 8 + base, 500);
                play_note(root + 10 + base, 500);
                play_note(root + 12 + base, 500);
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
