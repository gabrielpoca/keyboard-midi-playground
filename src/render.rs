use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::sleep;
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
    handle: thread::JoinHandle<()>,
}

impl Render {
    pub fn new(_base: Arc<Mutex<(u8)>>, logs_m: Arc<RwLock<(Vec<String>)>>) -> Render {
        let handle = thread::spawn(move || {
            Render::render(logs_m).unwrap();
        });

        return Render { handle };
    }

    fn render(logs_m: Arc<RwLock<(Vec<String>)>>) -> Result<(), Box<dyn Error>> {
        let mut user_input = String::new();
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        let events = Events::new();
        terminal.clear()?;

        loop {
            terminal.draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                    .split(f.size());

                Paragraph::new([Text::raw(&user_input)].iter())
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL).title("Input"))
                    .render(&mut f, chunks[1]);

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

            match events.next()? {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        //terminal.clear()?;
                        break;
                    }
                    Key::Char(e) => {
                        user_input.push(e);
                    }
                    Key::Backspace => {
                        user_input.pop();
                    }
                    _ => {}
                },
                _ => {}
            }

            //sleep(Duration::from_millis(50));
        }

        return Ok(());
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}
