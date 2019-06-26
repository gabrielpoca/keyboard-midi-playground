use crossbeam_channel::tick;
use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;

use super::events::{Event, Events};

pub struct Render {
    render_handle: thread::JoinHandle<()>,
    user_input_handle: thread::JoinHandle<()>,
}

impl Render {
    pub fn new(
        _base: Arc<Mutex<(u8)>>,
        logs_m: Arc<RwLock<(Vec<String>)>>,
        events_emitter: crossbeam_channel::Sender<String>,
        events_recv: crossbeam_channel::Receiver<String>,
    ) -> Render {
        let user_input_m = Arc::new(RwLock::new(String::new()));
        let user_input_render_m = user_input_m.clone();
        let user_input_input_m = user_input_m.clone();

        let render_handle = thread::spawn(move || {
            Render::handle_render(events_recv, logs_m, user_input_render_m).unwrap();
        });

        let user_input_handle = thread::spawn(move || {
            Render::handle_user_input(events_emitter, user_input_input_m).unwrap();
        });

        return Render {
            render_handle,
            user_input_handle,
        };
    }

    fn handle_user_input(
        events_emitter: crossbeam_channel::Sender<String>,
        user_input_m: Arc<RwLock<String>>,
    ) -> Result<(), Box<dyn Error>> {
        let events = Events::new();

        loop {
            match events.next()? {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        events_emitter.send("quit".into()).unwrap();
                        break;
                    }
                    Key::Char(e) => {
                        let mut user_input = user_input_m.write().unwrap();
                        user_input.push(e);
                    }
                    Key::Backspace => {
                        let mut user_input = user_input_m.write().unwrap();
                        user_input.pop();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        return Ok(());
    }

    fn handle_render(
        events_recv: crossbeam_channel::Receiver<String>,
        logs_m: Arc<RwLock<(Vec<String>)>>,
        user_input_m: Arc<RwLock<(String)>>,
    ) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;
        let render_ticker = tick(Duration::from_millis(100));

        loop {
            select! {
                recv(render_ticker) -> _ => {
                    terminal.draw(|mut f| {
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(1)
                            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                            .split(f.size());

                        {
                            let user_input: RwLockReadGuard<String> = user_input_m.read().unwrap().into();
                            Paragraph::new([Text::raw(&*user_input)].iter())
                                .style(Style::default().fg(Color::Yellow))
                                .block(Block::default().borders(Borders::ALL).title("Input"))
                                .render(&mut f, chunks[1]);
                        }

                        let all = logs_m.read().unwrap();
                        let items = all.iter().map(|item| {
                            Text::styled(format!("{}", item), Style::default().fg(Color::White))
                        });

                        List::new(items)
                            .block(Block::default().title("Block 2").borders(Borders::ALL))
                            .start_corner(Corner::BottomLeft)
                            .render(&mut f, chunks[0]);
                    })?;

                    io::stdout().flush().ok();
                }
                recv(events_recv) -> msg => {
                    match msg.unwrap().as_ref() {
                        "quit" => {
                            terminal.clear()?;
                            break;
                        },
                        _ => {}
                    }
                }
            }
        }

        return Ok(());
    }

    pub fn wait(self) {
        self.render_handle.join().unwrap_or_else(|_error| {
            return;
        });

        self.user_input_handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
