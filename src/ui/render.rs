extern crate sdl2;

use crate::app_state::*;
use crate::events;
use crate::events::EventBus;
use crate::scale::chord;
use crate::scale::*;
use crossbeam_channel::{Receiver, Sender};
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::error::Error;
use std::sync::*;
use std::time::Duration;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct Render {
    pub recv: Receiver<events::Event>,
    pub emitter: Sender<events::Event>,
    pub app_state: Arc<RwLock<AppState>>,
}

impl Render {
    pub fn new(app_state: Arc<RwLock<AppState>>, event_bus: &mut EventBus) -> Self {
        let events_recv = event_bus.new_receive();

        return Render {
            app_state,
            recv: events_recv,
            emitter: event_bus.emitter.clone(),
        };
    }

    pub fn handle(&self) -> Result<(), Box<dyn Error>> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("awesome midi player", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.present();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("./assets/Roboto/Roboto-Regular.ttf", 128)?;
        let texture_creator = canvas.texture_creator();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let surface = font
                .render("Natural Minor")
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = texture.query();
            let target = rect!(0, 0, width, height);

            canvas.copy(&texture, None, Some(target))?;

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    Event::KeyDown {
                        repeat: false,
                        keycode,
                        ..
                    } => match keycode {
                        Some(Keycode::A {}) | Some(Keycode::S) | Some(Keycode::D)
                        | Some(Keycode::F) | Some(Keycode::G) | Some(Keycode::H)
                        | Some(Keycode::J) | Some(Keycode::K) => {
                            self.handle_key_on(keycode.unwrap());
                        }
                        Some(Keycode::Num1 {}) => {
                            let scale = HarmonicMinor::new(60);
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.set_scale(Box::new(scale));
                        }
                        Some(Keycode::Q {}) => {
                            self.emitter.send(events::Event::Quit)?;
                            break 'running;
                        }
                        _ => {}
                    },
                    Event::KeyUp {
                        repeat: false,
                        keycode,
                        ..
                    } => match keycode {
                        Some(Keycode::A {}) | Some(Keycode::S) | Some(Keycode::D)
                        | Some(Keycode::F) | Some(Keycode::G) | Some(Keycode::H)
                        | Some(Keycode::J) | Some(Keycode::K) => {
                            self.handle_key_off(keycode.unwrap());
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

    fn handle_key_on(&self, keycode: Keycode) {
        info!("ON {}", keycode);
        let notes = self.keycode_to_midi(keycode);

        for note in notes.iter() {
            self.emitter
                .send(events::Event::Note {
                    message: events::NoteMessage::On,
                    note: *note as u8,
                    velocity: 10,
                })
                .unwrap();
        }
    }

    fn handle_key_off(&self, keycode: Keycode) {
        info!("OFF {}", keycode);
        let notes = self.keycode_to_midi(keycode);

        for note in notes.iter() {
            self.emitter
                .send(events::Event::Note {
                    message: events::NoteMessage::Off,
                    note: *note as u8,
                    velocity: 10,
                })
                .unwrap();
        }
    }

    fn keycode_to_midi(&self, keycode: Keycode) -> Vec<u32> {
        let index = match keycode {
            Keycode::A => 0,
            Keycode::S => 1,
            Keycode::D => 2,
            Keycode::F => 3,
            Keycode::G => 4,
            Keycode::H => 5,
            Keycode::J => 6,
            Keycode::K => 7,
            _ => 0,
        };

        let app_state = self.app_state.read().unwrap();
        let scale = &app_state.scale;

        return chord::get(scale, scale.note(index));
    }
}
