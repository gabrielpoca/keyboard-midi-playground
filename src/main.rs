#[macro_use]
extern crate crossbeam_channel;
extern crate env_logger;
extern crate log;

use player::Metronome;
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
    let mut metronome = Metronome::new(event_bus.new_receive(), 50);

    let scale: NaturalMinor = Scale::new(48);
    let notes = [
        scale.note(-3),
        scale.note(0),
        scale.note(-3),
        scale.note(0),
        scale.note(4),
        scale.note(2),
        scale.note(3),
        scale.note(1),
    ];

    let output = Output::new(event_bus.new_receive());
    let player = Player::new(
        app_state.clone(),
        metronome.new_receive(),
        event_bus.emitter.clone(),
        event_bus.new_receive(),
        true,
        notes.to_vec(),
        scale,
    );

    let mut render = Render::new(app_state.clone(), &mut event_bus);

    event_bus.start();
    metronome.start();

    render.handle().unwrap();

    println!("Stopping");

    player.wait();
    output.wait();
    event_bus.wait();

    Ok(())
}
