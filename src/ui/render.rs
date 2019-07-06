use super::canvas;
use crate::events::{Event, EventBus};
use crossbeam_channel::{Receiver, Sender};

pub struct Render {
    pub recv: Receiver<Event>,
    pub emitter: Sender<Event>,
}

impl Render {
    pub fn new(event_bus: &mut EventBus) -> Render {
        let events_recv = event_bus.new_receive();
        return Render {
            recv: events_recv,
            emitter: event_bus.emitter.clone(),
        };
    }

    pub fn start(&self) {
        canvas::handle(&self).unwrap();
    }
}
