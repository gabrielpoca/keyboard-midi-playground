mod keyboard;

use crate::app_state::*;
use crate::events::*;
use crate::scale::*;
use std::sync::*;
use std::thread;

use keyboard::KeyboardToMidi;

pub struct Player {
    handle: thread::JoinHandle<()>,
}

impl Player {
    pub fn new(app_state: Arc<RwLock<AppState>>, event_bus: &mut EventBus) -> Self {
        let events_recv = event_bus.new_receive();

        let keyboard_to_midi = KeyboardToMidi {
            emitter: event_bus.emitter.clone(),
            app_state: app_state.clone(),
        };

        let handle = thread::spawn(move || loop {
            select! {
                recv(events_recv) -> msg => {
                    let event = msg.unwrap_or_else({ |_| Event::None });

                    match event {
                        Event::Quit { } => {
                            break;
                        }
                        Event::KeyDown(key) => {
                            match key {
                                Key::Num1 {} => {
                                    keyboard_to_midi.reset();
                                    let scale = NaturalMinor::new(60);
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.set_scale(Box::new(scale));
                                    app_state
                                    .pressed_keys
                                    .insert(key, true);
                                }
                                Key::Num2 {} => {
                                    keyboard_to_midi.reset();
                                    let scale = HarmonicMinor::new(60);
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.set_scale(Box::new(scale));
                                    app_state
                                    .pressed_keys
                                    .insert(key, true);
                                }
                                Key::Num3 {} => {
                                    keyboard_to_midi.reset();
                                    let scale = MelodicMinor::new(60);
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.set_scale(Box::new(scale));
                                    app_state
                                    .pressed_keys
                                    .insert(key, true);
                                }
                                Key::Z {} => {
                                    keyboard_to_midi.reset();
                                    let mut app_state = app_state.write().unwrap();

                                    let scale = &mut app_state.scale;
                                    scale.decrease_root(12);
                                    app_state.pressed_keys.insert(key, true);

                                }
                                Key::X {} => {
                                    keyboard_to_midi.reset();
                                    let mut app_state = app_state.write().unwrap();

                                    let scale = &mut app_state.scale;
                                    scale.increase_root(12);
                                    app_state.pressed_keys.insert(key, true);
                                }
                                Key::Space {} => {
                                    keyboard_to_midi.reset();
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.toggle_play_mode();
                                }
                                Key::C {} => {
                                    keyboard_to_midi.reset();
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.scale.decrease_root(1);
                                }
                                Key::V {} => {
                                    keyboard_to_midi.reset();
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.scale.increase_root(2);
                                }
                                Key::W | Key::E | Key::R
                                | Key::T | Key::Y | Key::U
                                | Key::I | Key::O | Key::A
                                | Key::S | Key::D | Key::F
                                | Key::G | Key::H | Key::J
                                | Key::K | Key::L => {
                                    keyboard_to_midi.handle_key_on(key.clone());
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.pressed_keys.insert(key, true);
                                }
                                _ =>  {
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.pressed_keys.insert(key, true);
                                }
                            }
                        }
                        Event::KeyUp(key) => {
                            match key {
                                Key::W | Key::E | Key::R
                                | Key::T | Key::Y | Key::U
                                | Key::I | Key::O | Key::A
                                | Key::S | Key::D | Key::F
                                | Key::G | Key::H | Key::J
                                | Key::K | Key::L => {
                                    keyboard_to_midi.handle_key_off(key.clone());
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.pressed_keys.remove(&key);
                                }
                                _ => {
                                    let mut app_state = app_state.write().unwrap();
                                    app_state.pressed_keys.remove(&key);
                                }
                            }
                        }
                        _ => {}
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
