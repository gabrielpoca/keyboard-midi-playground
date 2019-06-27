use super::events::Event;
use midir::MidiOutput;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Output {
    handle: thread::JoinHandle<()>,
}

impl Output {
    pub fn new(
        events_recv: crossbeam_channel::Receiver<Event>,
        logs_m: Arc<RwLock<(Vec<String>)>>,
    ) -> Output {
        let out_port = Output::get_port(Arc::clone(&logs_m)).unwrap();

        let handle = thread::spawn(move || {
            let midi_out = MidiOutput::new("Midi seq").unwrap();

            {
                let mut logs = logs_m.write().unwrap();

                let midi_out_name = midi_out.port_name(out_port).unwrap();

                logs.insert(0, "Connection open".into());
                logs.insert(0, format!("Writing to device {}", midi_out_name,));
            }

            let mut conn_out = midi_out.connect(out_port, "midi-seq").unwrap();

            loop {
                select! {
                    recv(events_recv) -> msg => {
                        match msg.unwrap() {
                            Event::Note {
                                message,
                                note,
                                velocity,
                            } => {
                                conn_out.send(&[message as u8, note, velocity]).unwrap();
                                let mut logs = logs_m.write().unwrap();
                                logs.insert(0, format!("note: {}", note));
                            },
                            Event::Signal { message } => {
                                if message == "quit" {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            {
                let mut logs = logs_m.write().unwrap();
                logs.insert(0, "Closing connection".into());
                conn_out.close();
                logs.insert(0, "Connection closed".into());
            }
        });

        return Output { handle };
    }

    fn get_port(logs_m: Arc<RwLock<(Vec<String>)>>) -> Result<usize, Box<Error>> {
        let midi_out = MidiOutput::new("My Test Output").unwrap();

        let out_port = match midi_out.port_count() {
            0 => return Err("no output port found".into()),
            _ => 0,
        };

        {
            // Get an output port (read from console if multiple are available)

            let mut logs = logs_m.write().unwrap();
            for i in 0..midi_out.port_count() {
                logs.insert(
                    0,
                    format!("Midi out {} {}", midi_out.port_name(i).unwrap(), i),
                );
            }
        }
        //let out_port = match midi_out.port_count() {
        //0 => return Err("no output port found".into()),
        //1 => {
        //println!(
        //"Choosing the only available output port: {}",
        //midi_out.port_name(0).unwrap()
        //);
        //0
        //}
        //_ => {
        //println!("\nAvailable output ports:");
        //for i in 0..midi_out.port_count() {
        //println!("{}: {}", i, midi_out.port_name(i).unwrap());
        //}
        //print!("Please select output port: ");
        //stdout().flush()?;
        //let mut input = String::new();
        //stdin().read_line(&mut input)?;
        //input.trim().parse()?
        //}
        //};

        return Ok(out_port);
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
