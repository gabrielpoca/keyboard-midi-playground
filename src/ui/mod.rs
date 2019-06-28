use std::sync::{Arc, RwLock};
use std::thread;

use crate::events::EventBus;

mod input;
mod terminal;
mod user_events;

pub struct Render {
    render_handle: thread::JoinHandle<()>,
    user_input_handle: thread::JoinHandle<()>,
}

impl Render {
    pub fn new(logs_m: Arc<RwLock<(Vec<String>)>>, event_bus: &mut EventBus) -> Render {
        let user_input_m = Arc::new(RwLock::new(String::new()));
        let user_input_render_m = user_input_m.clone();
        let user_input_input_m = user_input_m.clone();

        let render_events_recv = event_bus.new_receive();
        let user_input_events_recv = event_bus.new_receive();
        let user_input_events_emitter = event_bus.emitter.clone();

        let render_handle = thread::spawn(move || {
            terminal::handle(render_events_recv, logs_m, user_input_render_m).unwrap();
        });

        let user_input_handle = thread::spawn(move || {
            input::handle(
                user_input_events_emitter,
                user_input_events_recv,
                user_input_input_m,
            )
            .unwrap();
        });

        return Render {
            render_handle,
            user_input_handle,
        };
    }

    pub fn wait(self) {
        self.render_handle.join().unwrap_or_else(|_error| {
            return;
        });

        self.user_input_handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
