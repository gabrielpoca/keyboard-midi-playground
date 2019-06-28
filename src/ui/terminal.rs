use crate::events::Event;
use crossbeam_channel::tick;
use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};
use tui::Terminal;

struct Cursor {
    row: usize,
    column: usize,
}

struct State<'a> {
    cursor: Cursor,
    grid: Vec<Vec<&'a str>>,
}

pub fn handle(
    events_recv: crossbeam_channel::Receiver<Event>,
    logs_m: Arc<RwLock<(Vec<String>)>>,
    _user_input_m: Arc<RwLock<(String)>>,
) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    let render_ticker = tick(Duration::from_millis(80));

    let mut state = State {
        cursor: Cursor { row: 0, column: 0 },
        grid: vec![
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        ],
    };

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

                            let row = &state.grid[i];

                            for (index, _cell) in row.iter().enumerate() {
                                if state.cursor.row == i && state.cursor.column == index {
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
                match msg? {
                    Event::CursorUp { } => {
                        if state.cursor.row != 0 {
                            state.cursor.row -= 1;
                        }
                    },
                    Event::CursorDown { } => {
                        if state.cursor.row != 9 {
                            state.cursor.row += 1;
                        }
                    },
                    Event::CursorLeft { } => {
                        if state.cursor.column != 0 {
                            state.cursor.column -= 1;
                        }
                    },
                    Event::CursorRight { } => {
                        if state.cursor.column != 9 {
                            state.cursor.column += 1;
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
