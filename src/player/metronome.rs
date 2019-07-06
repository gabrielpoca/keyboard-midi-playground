use crate::events::Event;
use crossbeam_channel::{tick, unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
struct MetronomeInner {
    recv: Receiver<Event>,
    tick: Receiver<Instant>,
    all_emitters: Vec<Sender<Event>>,
}

#[derive(Debug)]
pub struct Metronome {
    inner: Arc<Mutex<MetronomeInner>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Metronome {
    pub fn new(recv: Receiver<Event>, speed: u64) -> Metronome {
        return Metronome {
            inner: Arc::new(Mutex::new(MetronomeInner {
                recv,
                tick: tick(Duration::from_millis(speed)),
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
                    recv(inner.recv) -> msg => {
                        match msg.unwrap() {
                            Event::Quit => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    recv(inner.tick) -> _ => {
                        let my_event = Event::Tick;
                        for e in inner.all_emitters.clone() {
                            match e.send(my_event.clone()) {
                                Err(_) => {},
                                _ => {}
                            }
                        };
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
