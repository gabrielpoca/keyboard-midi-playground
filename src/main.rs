#[macro_use]
extern crate crossbeam_channel;
extern crate env_logger;
extern crate log;

use log::info;
use std::error::Error;

mod app_state;
mod events;
mod output;
mod scale;
mod ui;

use app_state::*;
use events::EventBus;
use output::Output;
use scale::NaturalMinor;
use scale::Scale;
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
    let mut app_state = Arc::new(RwLock::new(AppState::new()));
    let mut event_bus = EventBus::new();

    let scale: NaturalMinor = Scale::new(48);

    let output = Output::new(event_bus.new_receive());

    let mut render = Render::new(app_state.clone(), scale.clone(), &mut event_bus);

    event_bus.start();

    render.handle().unwrap();

    info!("Stopping");

    output.wait();
    event_bus.wait();

    Ok(())
}
