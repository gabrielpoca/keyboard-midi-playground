#[macro_use]
extern crate crossbeam_channel;
extern crate env_logger;
extern crate log;

use log::info;
use std::error::Error;

mod app_state;
mod events;
mod output;
mod player;
mod scale;
mod ui;

use app_state::*;
use events::EventBus;
use output::Output;
use player::Player;
use scale::*;
use std::sync::*;
use ui::Render;

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => println!("Done!"),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<Error>> {
    let scale = NaturalMinor::new(60);
    let app_state = Arc::new(RwLock::new(AppState::new(Box::new(scale))));
    let mut event_bus = EventBus::new();

    let output = Output::new(event_bus.new_receive());
    let render = Render::new(app_state.clone(), &mut event_bus);
    let player = Player::new(app_state.clone(), &mut event_bus);

    event_bus.start();

    render.handle().unwrap();

    info!("Stopping");

    output.wait();
    event_bus.wait();
    player.wait();

    Ok(())
}
