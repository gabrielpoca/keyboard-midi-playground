use super::message::Message;
use midir::MidiOutput;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub struct Output {
    handle: thread::JoinHandle<()>,
}

impl Output {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Output {
        let out_port = Output::get_port().unwrap();

        let handle = thread::spawn(move || {
            println!("\nOpening connection");
            let midi_out = MidiOutput::new("My Test Output").unwrap();
            let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
            println!("Connection open. Listen!");

            sleep(Duration::from_millis(150));

            for received in receiver {
                conn_out
                    .send(&[received.message, received.note, received.velocity])
                    .unwrap();
            }

            println!("\nClosing connection");
            // This is optional, the connection would automatically be closed as soon as it goes out of scope
            conn_out.close();
            println!("Connection closed");
        });

        return Output { handle };
    }

    fn get_port() -> Result<usize, Box<Error>> {
        let midi_out = MidiOutput::new("My Test Output").unwrap();

        // Get an output port (read from console if multiple are available)
        let out_port = match midi_out.port_count() {
            0 => return Err("no output port found".into()),
            1 => {
                println!(
                    "Choosing the only available output port: {}",
                    midi_out.port_name(0).unwrap()
                );
                0
            }
            _ => {
                println!("\nAvailable output ports:");
                for i in 0..midi_out.port_count() {
                    println!("{}: {}", i, midi_out.port_name(i).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                input.trim().parse()?
            }
        };

        return Ok(out_port);
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
