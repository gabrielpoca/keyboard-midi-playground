use super::event::Event;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct EventBusInner {
    local_receiver: Receiver<Event>,
    all_emitters: Vec<Sender<Event>>,
}

#[derive(Debug)]
pub struct EventBus {
    pub emitter: Sender<Event>,
    inner: Arc<Mutex<EventBusInner>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl EventBus {
    pub fn new() -> EventBus {
        let (events_emitter, events_recv) = unbounded();

        return EventBus {
            emitter: events_emitter,
            inner: Arc::new(Mutex::new(EventBusInner {
                local_receiver: events_recv,
                all_emitters: Vec::new(),
            })),
            handle: None,
        };
    }

    pub fn new_receive(&mut self) -> crossbeam_channel::Receiver<Event> {
        let (events_emitter, events_recv) = unbounded();

        let mut inner = self.inner.lock().unwrap();

        inner.all_emitters.push(events_emitter);

        return events_recv;
    }

    pub fn start(&mut self) {
        let innert_copy = self.inner.clone();

        let handle = thread::spawn(move || {
            let inner = &innert_copy.lock().unwrap();

            loop {
                select! {
                    recv(inner.local_receiver) -> event => {
                        let my_event = event.unwrap();
                        for e in inner.all_emitters.clone() {
                            match e.send(my_event.clone()) {
                                Err(_) => {},
                                _ => {}
                            }
                        };

                        match my_event {
                            Event::Quit => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        self.handle = Some(handle);
    }

    pub fn wait(self) {
        self.handle.unwrap().join().unwrap_or_else(|_error| {
            return;
        });
    }
}
