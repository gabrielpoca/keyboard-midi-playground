use super::events::Event;
use midir::MidiOutput;
use std::error::Error;
use std::thread;

pub struct Output {
    handle: thread::JoinHandle<()>,
}

impl Output {
    pub fn new(events_recv: crossbeam_channel::Receiver<Event>) -> Output {
        let out_port = Output::get_port().unwrap();

        let handle = thread::spawn(move || {
            let midi_out = MidiOutput::new("Midi seq").unwrap();
            let mut conn_out = midi_out.connect(out_port, "midi-seq").unwrap();

            loop {
                select! {
                    recv(events_recv) -> msg => {
                        match msg.unwrap_or_else({|_| Event::None }) {
                            Event::Note {
                                message,
                                note,
                                velocity,
                            } => {
                                conn_out.send(&[message as u8, note, velocity]).unwrap();
                            },
                            Event::Quit { } => {
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }

            conn_out.close();
        });

        return Output { handle };
    }

    fn get_port() -> Result<usize, Box<Error>> {
        let midi_out = MidiOutput::new("My Test Output").unwrap();

        let out_port = match midi_out.port_count() {
            0 => return Err("no output port found".into()),
            _ => 0,
        };

        return Ok(out_port);
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
