use super::Render;
use crate::events;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

struct InputState {
    mode: u8,
    mappings: HashMap<char, u32>,
}

impl InputState {
    fn new() -> InputState {
        let mut mappings = HashMap::new();
        mappings.insert('a', 0);
        mappings.insert('s', 1);
        mappings.insert('d', 2);
        mappings.insert('f', 3);
        mappings.insert('g', 4);
        mappings.insert('h', 5);
        mappings.insert('j', 6);
        mappings.insert('k', 7);
        mappings.insert('l', 8);
        mappings.insert(';', 9);

        return InputState { mode: 0, mappings };
    }

    fn edit_mode(&mut self) {
        self.mode = 1;
    }

    fn play_mode(&mut self) {
        self.mode = 0;
    }

    fn is_play_mode(&self) -> bool {
        return self.mode == 0;
    }

    fn transform_key(&self, key: char) -> u32 {
        if self.mappings.contains_key(&key.into()) {
            match self.mappings.get(&key.into()) {
                Some(&n) => return n,
                None => return 0,
            }
        } else {
            return 0;
        }
    }
}

pub fn handle(render: &Render) -> Result<(), Box<dyn Error>> {
    let input_state = InputState::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Up {}) => {}
                    Some(Keycode::Down {}) => {}
                    Some(Keycode::Left {}) => {}
                    Some(Keycode::Right {}) => {}
                    Some(Keycode::Space {}) => {
                        render.emitter.send(events::Event::Pause)?;
                    }
                    Some(Keycode::Q {}) => {
                        render.emitter.send(events::Event::Quit)?;
                        break 'running;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
