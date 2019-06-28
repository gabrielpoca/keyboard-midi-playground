use crate::events::Event;
use crossbeam_channel::tick;
use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Row, Table, Text, Widget};
use tui::Terminal;

use crate::events::EventBus;
use crate::user_events::UserEvents;

pub struct Render {
    render_handle: thread::JoinHandle<()>,
    user_input_handle: thread::JoinHandle<()>,
}

struct Cursor {
    row: usize,
    column: usize,
}

impl Render {
    pub fn new(logs_m: Arc<RwLock<(Vec<String>)>>, event_bus: &mut EventBus) -> Render {
        let user_input_m = Arc::new(RwLock::new(String::new()));
        let user_input_render_m = user_input_m.clone();
        let user_input_input_m = user_input_m.clone();

        let render_events_recv = event_bus.new_receive();
        let user_input_events_recv = event_bus.new_receive();
        let user_input_events_emitter = event_bus.emitter.clone();

        let render_handle = thread::spawn(move || {
            Render::handle_render(render_events_recv, logs_m, user_input_render_m).unwrap();
        });

        let user_input_handle = thread::spawn(move || {
            Render::handle_user_input(
                user_input_events_emitter,
                user_input_events_recv,
                user_input_input_m,
            )
            .unwrap();
        });

        return Render {
            render_handle,
            user_input_handle,
        };
    }

    fn handle_user_input(
        events_emitter: crossbeam_channel::Sender<Event>,
        events_recv: crossbeam_channel::Receiver<Event>,
        user_input_m: Arc<RwLock<String>>,
    ) -> Result<(), Box<dyn Error>> {
        UserEvents::new(events_emitter.clone());

        loop {
            select! {
            recv(events_recv) -> msg => {
                match msg.unwrap() {
                Event::Quit {} => {
                    break;
                }
                Event::Key { key } => match key {
                    Key::Char('q') => {
                        events_emitter.send(Event::Quit{}).unwrap();
                    }
                    Key::Char('h') => {
                        events_emitter.send(Event::CursorLeft{}).unwrap();
                    }
                    Key::Char('l') => {
                        events_emitter.send(Event::CursorRight{}).unwrap();
                    }
                    Key::Char('j') => {
                        events_emitter.send(Event::CursorDown{}).unwrap();
                    }
                    Key::Char('k') => {
                        events_emitter.send(Event::CursorUp{}).unwrap();
                    }
                    Key::Char(' ') => {
                        events_emitter.send(Event::Pause{}).unwrap();
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
            }
        }

        return Ok(());
    }

    fn handle_render(
        events_recv: crossbeam_channel::Receiver<Event>,
        logs_m: Arc<RwLock<(Vec<String>)>>,
        user_input_m: Arc<RwLock<(String)>>,
    ) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;
        let render_ticker = tick(Duration::from_millis(80));

        let mut cursor = Cursor { row: 0, column: 0 };

        loop {
            select! {
                recv(render_ticker) -> _ => {
                    terminal.draw(|mut f| {
                        let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .margin(1)
                            .constraints([Constraint::Length(20), Constraint::Percentage(90)].as_ref())
                            .split(f.size());

                        {
                            let items = vec![
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                                vec!["CH 0", "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9"],
                            ];

                            let chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                             Constraint::Percentage(10),
                                ].as_ref())
                                .split(chunks[1]);

                            for i in 0..10 {
                                let chunks = Layout::default()
                                    .direction(Direction::Horizontal)
                                    .constraints([
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                                 Constraint::Percentage(10),
                                    ].as_ref())
                                    .split(chunks[i]);

                                let row = &items[i];

                                for (index, _cell) in row.iter().enumerate() {
                                    if cursor.row == i && cursor.column == index {
                                        Block::default()
                                            .borders(Borders::ALL)
                                            .style(Style::default().bg(Color::Green))
                                            .render(&mut f, chunks[index]);
                                    } else {
                                        Block::default()
                                            .borders(Borders::ALL)
                                            .render(&mut f, chunks[index]);
                                    }
                                }
                            }
                        }

                        {
                            let all = logs_m.read().unwrap();

                            let items = all.iter().map(|item| {
                                    Text::styled(format!("{}", item), Style::default().fg(Color::White))
                            });

                            List::new(items)
                                .block(Block::default().borders(Borders::ALL))
                                .start_corner(Corner::BottomLeft)
                                .render(&mut f, chunks[0]);
                        }
                    })?;

                    io::stdout().flush().ok();
                },
                recv(events_recv) -> msg => {
                    match msg.unwrap() {
                        Event::CursorUp { } => {
                            if cursor.row != 0 {
                                cursor.row -= 1;
                            }
                        },
                        Event::CursorDown { } => {
                            if cursor.row != 9 {
                                cursor.row += 1;
                            }
                        },
                        Event::CursorLeft { } => {
                            if cursor.column != 0 {
                                cursor.column -= 1;
                            }
                        },
                        Event::CursorRight { } => {
                            if cursor.column != 9 {
                                cursor.column += 1;
                            }
                        },
                        Event::Quit { } => {
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
