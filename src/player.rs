use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use super::message::Message;

pub struct Player {
    handle: thread::JoinHandle<()>,
}

impl Player {
    pub fn new(conn: Arc<Mutex<(u8)>>, sender: mpsc::Sender<Message>) -> Player {
        let handle = thread::spawn(move || {
            let play_note = |note: u8, duration: u64| {
                const NOTE_ON_MSG: u8 = 0x90;
                const NOTE_OFF_MSG: u8 = 0x80;
                const VELOCITY: u8 = 0x64;
                let _ = sender
                    .send(Message {
                        message: NOTE_ON_MSG,
                        note,
                        velocity: VELOCITY,
                    })
                    .unwrap();
                sleep(Duration::from_millis(duration * 150));
                let _ = sender
                    .send(Message {
                        message: NOTE_OFF_MSG,
                        note,
                        velocity: VELOCITY,
                    })
                    .unwrap();
            };

            for _i in 1..4 {
                let num = conn.lock().unwrap();
                let base = *num;
                std::mem::drop(num);
                println!("base {}", base);
                play_note(66 + base, 4);
                play_note(65 + base, 3);
                play_note(63 + base, 1);
                play_note(61 + base, 6);
                play_note(59 + base, 2);
                play_note(58 + base, 4);
                play_note(56 + base, 4);
                play_note(54 + base, 4);
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
