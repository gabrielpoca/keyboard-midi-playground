use std::io;
use std::thread;

use crate::events::Event;
use termion::event::Key;
use termion::input::TermRead;

pub struct UserEvents {
    handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
        }
    }
}

impl UserEvents {
    pub fn new(emitter: crossbeam_channel::Sender<Event>) -> UserEvents {
        let config = Config::default();

        let handle = {
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    match evt {
                        Ok(key) => {
                            if let Err(_) = emitter.send(Event::Key { key }) {
                                return;
                            }

                            if key == config.exit_key {
                                return;
                            }
                        }
                        Err(_) => {}
                    }
                }
            })
        };

        UserEvents { handle }
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
