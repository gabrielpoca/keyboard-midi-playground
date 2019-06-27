#[macro_use]
extern crate crossbeam_channel;

use std::error::Error;
use std::sync::{Arc, Mutex, RwLock};

mod events;
mod output;
mod player;
mod render;
mod user_events;

use events::EventBus;
use output::Output;
use player::Player;
use render::Render;

fn main() {
    match run() {
        Ok(_) => println!("Done!"),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<Error>> {
    let logs = Vec::new();
    let logs_m = Arc::new(RwLock::new(logs));
    let m = Arc::new(Mutex::new(0));

    let mut event_bus = EventBus::new();

    let output = Output::new(event_bus.new_receive(), Arc::clone(&logs_m));
    let player = Player::new(Arc::clone(&m), event_bus.emitter.clone());
    let render = Render::new(Arc::clone(&m), Arc::clone(&logs_m), &mut event_bus);

    event_bus.start();

    player.wait();
    output.wait();
    render.wait();
    event_bus.wait();

    Ok(())
}
