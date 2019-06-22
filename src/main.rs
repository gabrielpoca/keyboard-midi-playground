use std::error::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use input::Input;
use message::Message;
use output::Output;
use player::Player;

mod input;
mod message;
mod output;
mod player;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}

fn run() -> Result<(), Box<Error>> {
    let (tx, rx) = mpsc::channel::<Message>();
    let m = Arc::new(Mutex::new(0));

    let output = Output::new(rx);
    let player = Player::new(Arc::clone(&m), tx);
    let input = Input::new(Arc::clone(&m));

    player.wait();
    output.wait();
    input.wait();

    Ok(())
}
