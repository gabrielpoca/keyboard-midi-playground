use std::error::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};

use message::Message;
use output::Output;
use player::Player;
use render::Render;

mod events;
mod message;
mod output;
mod player;
mod render;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<Error>> {
    let (note_sender, note_recv) = mpsc::channel::<Message>();
    let logs = Vec::new();
    let logs_m = Arc::new(RwLock::new(logs));
    let m = Arc::new(Mutex::new(0));

    let output = Output::new(note_recv, Arc::clone(&logs_m));
    let player = Player::new(Arc::clone(&m), note_sender);
    let render = Render::new(Arc::clone(&m), Arc::clone(&logs_m));

    player.wait();
    output.wait();
    render.wait();

    Ok(())
}
