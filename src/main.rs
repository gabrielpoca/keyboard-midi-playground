#[macro_use]
extern crate crossbeam_channel;

use std::error::Error;
use std::sync::{Arc, RwLock};

mod events;
mod output;
mod player;
mod ui;

use events::EventBus;
use output::Output;
use player::Player;
use ui::Render;

fn main() {
    match run() {
        Ok(_) => println!("Done!"),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<Error>> {
    let logs = Vec::new();
    let logs_m = Arc::new(RwLock::new(logs));

    let mut event_bus = EventBus::new();

    let output = Output::new(event_bus.new_receive(), Arc::clone(&logs_m));
    let player = Player::new(event_bus.emitter.clone(), event_bus.new_receive());
    let render = Render::new(Arc::clone(&logs_m), &mut event_bus);

    event_bus.start();

    player.wait();
    output.wait();
    render.wait();
    event_bus.wait();

    Ok(())
}
