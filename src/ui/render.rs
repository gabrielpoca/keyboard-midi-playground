extern crate sdl2;

use super::keyboard::KeyboardToMidi;
use crate::app_state::*;
use crate::events;
use crate::events::EventBus;
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

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn get_centered_rect(
    rect_width: u32,
    rect_height: u32,
    cons_width: u32,
    cons_height: u32,
    cx: i32,
    cy: i32,
) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as u32;
            (cons_width as u32, h)
        } else {
            let w = (rect_width as f32 / hr) as u32;
            (w, cons_height as u32)
        }
    } else {
        (rect_width as u32, rect_height as u32)
    };

    let cx = cx;
    let cy = cy;

    let cx = (cx as u32) + (cons_width - w) / 2;
    let cy = (cy as u32) + (cons_width - h) / 2;

    rect!(cx as i32, cy as i32, w, h)
}

fn get_top_left_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    rect!(22, 22, w, h)
}

static ROW_LENGTH: u32 = 10;

static FIRST_ROW: [&str; 10] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];

static SECOND_ROW: [&str; 9] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];

static THIRD_ROW: [&str; 7] = ["Z", "X", "C", "V", "B", "N", "M"];

fn get_keyboard_rects<'a>(cons_width: u32, _cons_height: u32) -> (Vec<Rect>, Vec<(Rect, &'a str)>) {
    let spacing = 10;
    let total_spacing = 8 * spacing;
    let individual_width = (cons_width - total_spacing) / ROW_LENGTH;

    let mut res = Vec::new();
    let mut keycode = Vec::new();

    for (i, item) in FIRST_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {20 + (i * individual_width) + (i * spacing), 200, individual_width, individual_width};
        keycode.push((target, *item));
        res.push(target);
    }

    for (i, item) in SECOND_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {28 + (i * individual_width) + (i * spacing), 200 + individual_width + spacing, individual_width, individual_width};
        keycode.push((target, *item));
        res.push(target);
    }

    for (i, item) in THIRD_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {36 + (i * individual_width) + (i * spacing), 200 + (individual_width + spacing) * 2, individual_width, individual_width};
        keycode.push((target, *item));
        res.push(target);
    }

    return (res, keycode);
}

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
            .window("awesome midi player", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.present();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("./assets/Roboto/Roboto-Regular.ttf", 28)?;
        let texture_creator = canvas.texture_creator();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let keyboard_to_midi = KeyboardToMidi {
            emitter: self.emitter.clone(),
            app_state: self.app_state.clone(),
        };

        'running: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            {
                let app_state = self.app_state.write().unwrap();
                let scale = &app_state.scale;

                let surface = font
                    .render(&scale.label())
                    .blended(Color::RGBA(255, 255, 255, 255))
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                let TextureQuery { width, height, .. } = texture.query();
                //let target = get_top_left_rect(width, height, SCREEN_WIDTH / 3, SCREEN_HEIGHT / 3);
                let target = rect!(20, 20, width, height);

                canvas.copy(&texture, None, Some(target))?;
            }

            canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
            let (rects, keycodes) = get_keyboard_rects(SCREEN_WIDTH - 60, SCREEN_HEIGHT);

            canvas.draw_rects(&rects)?;

            {
                let app_state = self.app_state.write().unwrap();

                for (target, key) in keycodes {
                    let color = Color::RGBA(255, 255, 255, 255);

                    match app_state.pressed_keys.get(key) {
                        Some(true) => {
                            canvas.set_draw_color(Color::RGBA(0, 255, 255, 255));
                            canvas.fill_rect(target)?;
                        }
                        _ => {}
                    }

                    let surface = font
                        .render(&key)
                        .blended(color)
                        .map_err(|e| e.to_string())?;

                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;

                    let TextureQuery { width, height, .. } = texture.query();
                    let target = get_centered_rect(
                        width,
                        height,
                        target.width(),
                        target.height(),
                        target.x,
                        target.y,
                    );

                    canvas.copy(&texture, None, Some(target))?;
                }
            }

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
                        Some(Keycode::W {}) | Some(Keycode::E) | Some(Keycode::R)
                        | Some(Keycode::T {}) | Some(Keycode::Y) | Some(Keycode::U)
                        | Some(Keycode::I {}) | Some(Keycode::O) | Some(Keycode::A {})
                        | Some(Keycode::S) | Some(Keycode::D) | Some(Keycode::F)
                        | Some(Keycode::G) | Some(Keycode::H) | Some(Keycode::J)
                        | Some(Keycode::K) | Some(Keycode::L) => {
                            keyboard_to_midi.handle_key_on(keycode.unwrap());
                            let mut app_state = self.app_state.write().unwrap();
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        Some(Keycode::Num1 {}) => {
                            let scale = NaturalMinor::new(60);
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.set_scale(Box::new(scale));
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        Some(Keycode::Num2 {}) => {
                            let scale = HarmonicMinor::new(60);
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.set_scale(Box::new(scale));
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        Some(Keycode::Num3 {}) => {
                            let scale = MelodicMinor::new(60);
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.set_scale(Box::new(scale));
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        Some(Keycode::Q {}) => {
                            self.emitter.send(events::Event::Quit)?;
                            break 'running;
                        }
                        Some(Keycode::Z {}) => {
                            let mut app_state = self.app_state.write().unwrap();
                            let scale = &mut app_state.scale;
                            scale.decrease_root(12);
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        Some(Keycode::X {}) => {
                            let mut app_state = self.app_state.write().unwrap();
                            let scale = &mut app_state.scale;
                            scale.increase_root(12);
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        Some(Keycode::Space {}) => {
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.toggle_play_mode();
                        }
                        Some(Keycode::C {}) => {
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.scale.decrease_root(1);
                        }
                        Some(Keycode::V {}) => {
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.scale.increase_root(2);
                        }
                        Some(key) => {
                            info!("key {:?}", key);
                            let mut app_state = self.app_state.write().unwrap();
                            app_state
                                .pressed_keys
                                .insert(keycode.unwrap().to_string(), true);
                        }
                        _ => {}
                    },
                    Event::KeyUp {
                        repeat: false,
                        keycode,
                        ..
                    } => match keycode {
                        Some(Keycode::W {}) | Some(Keycode::E) | Some(Keycode::R)
                        | Some(Keycode::T {}) | Some(Keycode::Y) | Some(Keycode::U)
                        | Some(Keycode::I {}) | Some(Keycode::O) | Some(Keycode::A {})
                        | Some(Keycode::S) | Some(Keycode::D) | Some(Keycode::F)
                        | Some(Keycode::G) | Some(Keycode::H) | Some(Keycode::J)
                        | Some(Keycode::K) | Some(Keycode::L) => {
                            keyboard_to_midi.handle_key_off(keycode.unwrap());
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.pressed_keys.remove(&keycode.unwrap().to_string());
                        }
                        Some(key) => {
                            info!("key {:?}", key);
                            let mut app_state = self.app_state.write().unwrap();
                            app_state.pressed_keys.remove(&keycode.unwrap().to_string());
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
}
