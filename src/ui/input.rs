use super::user_events::UserEvents;
use crate::events::Event;
use std::error::Error;
use std::sync::{Arc, RwLock};
use termion::event::Key;

pub fn handle(
    events_emitter: crossbeam_channel::Sender<Event>,
    events_recv: crossbeam_channel::Receiver<Event>,
    user_input_m: Arc<RwLock<String>>,
) -> Result<(), Box<dyn Error>> {
    UserEvents::new(events_emitter.clone());

    loop {
        select! {
            recv(events_recv) -> msg => {
                match msg.unwrap() {
                    Event::Quit {} => {
                        break;
                    },
                    Event::Key { key } => match key {
                        Key::Char('q') => {
                            events_emitter.send(Event::Quit)?;
                        }
                        Key::Char('h') => {
                            events_emitter.send(Event::CursorLeft)?;
                        }
                        Key::Char('l') => {
                            events_emitter.send(Event::CursorRight)?;
                        }
                        Key::Char('j') => {
                            events_emitter.send(Event::CursorDown)?;
                        }
                        Key::Char('k') => {
                            events_emitter.send(Event::CursorUp)?;
                        }
                        Key::Char(' ') => {
                            events_emitter.send(Event::Pause)?;
                        }
                        Key::Char(e) => {
                            let mut user_input = user_input_m.write().unwrap();
                            user_input.push(e);
                        }
                        Key::Backspace => {
                            let mut user_input = user_input_m.write().unwrap();
                            user_input.pop();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    return Ok(());
}
