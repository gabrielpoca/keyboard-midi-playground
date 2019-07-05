#[macro_use]
extern crate crossbeam_channel;
extern crate env_logger;
extern crate log;

use player::Metronome;
use std::error::Error;
use std::sync::{Arc, RwLock};

mod events;
mod output;
mod player;
mod scale;
mod ui;

use events::EventBus;
use output::Output;
use scale::NaturalMinor;
use player::Player;
use scale::Scale;
use ui::Render;

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => println!("Done!"),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<Error>> {
    let logs = Vec::new();
    let logs_m = Arc::new(RwLock::new(logs));

    let mut event_bus = EventBus::new();
    let mut metronome = Metronome::new(50);

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
        scale.note(-3),
        scale.note(0),
        scale.note(3),
        scale.note(6),
        scale.note(8),
        scale.note(5),
        scale.note(3),
        scale.note(1),
    ];

    let output = Output::new(event_bus.new_receive(), Arc::clone(&logs_m));
    let player = Player::new(
        metronome.new_receive(),
        event_bus.emitter.clone(),
        event_bus.new_receive(),
        true,
        notes.to_vec(),
        scale,
    );

    let scale: NaturalMinor = Scale::new(84);
    let notes = [
        scale.note(-3),
        scale.note(0),
        scale.note(-3),
        scale.note(0),
        scale.note(4),
        scale.note(2),
        scale.note(3),
        scale.note(1),
        scale.note(-3),
        scale.note(0),
        scale.note(3),
        scale.note(6),
        scale.note(8),
        scale.note(5),
        scale.note(3),
        scale.note(1),
    ];

    let player2 = Player::new(
        metronome.new_receive(),
        event_bus.emitter.clone(),
        event_bus.new_receive(),
        false,
        notes.to_vec(),
        scale,
    );
    let render = Render::new(Arc::clone(&logs_m), &mut event_bus);

    event_bus.start();
    metronome.start();

    player.wait();
    player2.wait();
    output.wait();
    render.wait();
    event_bus.wait();

    Ok(())
}
