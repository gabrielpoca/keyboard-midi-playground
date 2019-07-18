extern crate sdl2;

use crate::app_state::*;
use crate::events;
use crate::events::EventBus;
use crate::player::KeyboardTransform;
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

struct KeyDraw<'a> {
    target: Rect,
    key: &'a events::Key,
    color: Color,
}

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

struct RenderKey {
    key: events::Key,
    variant: u32,
}

static ROW_LENGTH: u32 = 10;

static NUM_ROW: [RenderKey; 10] = [
    RenderKey {
        variant: 2,
        key: events::Key::Num1,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num2,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num3,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num4,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num5,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num6,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num7,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num8,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num9,
    },
    RenderKey {
        variant: 2,
        key: events::Key::Num0,
    },
];

static FIRST_ROW: [RenderKey; 10] = [
    RenderKey {
        variant: 4,
        key: events::Key::Q,
    },
    RenderKey {
        variant: 0,
        key: events::Key::W,
    },
    RenderKey {
        variant: 0,
        key: events::Key::E,
    },
    RenderKey {
        variant: 0,
        key: events::Key::R,
    },
    RenderKey {
        variant: 0,
        key: events::Key::T,
    },
    RenderKey {
        variant: 0,
        key: events::Key::Y,
    },
    RenderKey {
        variant: 0,
        key: events::Key::U,
    },
    RenderKey {
        variant: 0,
        key: events::Key::I,
    },
    RenderKey {
        variant: 0,
        key: events::Key::O,
    },
    RenderKey {
        variant: 0,
        key: events::Key::P,
    },
];

static SECOND_ROW: [RenderKey; 9] = [
    RenderKey {
        variant: 0,
        key: events::Key::A,
    },
    RenderKey {
        variant: 0,
        key: events::Key::S,
    },
    RenderKey {
        variant: 0,
        key: events::Key::D,
    },
    RenderKey {
        variant: 0,
        key: events::Key::F,
    },
    RenderKey {
        variant: 0,
        key: events::Key::G,
    },
    RenderKey {
        variant: 0,
        key: events::Key::H,
    },
    RenderKey {
        variant: 0,
        key: events::Key::J,
    },
    RenderKey {
        variant: 0,
        key: events::Key::K,
    },
    RenderKey {
        variant: 0,
        key: events::Key::L,
    },
];

static THIRD_ROW: [RenderKey; 7] = [
    RenderKey {
        variant: 2,
        key: events::Key::Z,
    },
    RenderKey {
        variant: 2,
        key: events::Key::X,
    },
    RenderKey {
        variant: 2,
        key: events::Key::C,
    },
    RenderKey {
        variant: 2,
        key: events::Key::V,
    },
    RenderKey {
        variant: 0,
        key: events::Key::B,
    },
    RenderKey {
        variant: 0,
        key: events::Key::N,
    },
    RenderKey {
        variant: 0,
        key: events::Key::M,
    },
];

static SPACE_ROW: RenderKey = RenderKey {
    variant: 2,
    key: events::Key::Space,
};

fn color_for_variant(variant: u32) -> Color {
    return match variant {
        1 => Color::RGBA(0, 255, 255, 255),
        2 => Color::RGBA(0, 255, 155, 255),
        4 => Color::RGBA(0, 0, 155, 255),
        _ => Color::RGBA(0, 200, 255, 255),
    };
}

fn get_keyboard_rects<'a>(cons_width: u32, _cons_height: u32) -> (Vec<Rect>, Vec<KeyDraw<'a>>) {
    let spacing = 10;
    let total_spacing = 8 * spacing;
    let individual_width = (cons_width - total_spacing) / ROW_LENGTH;

    let mut res = Vec::new();
    let mut keycode = Vec::new();

    for (i, RenderKey { variant, key }) in NUM_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {20 + (i * individual_width) + (i * spacing), 150, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyDraw { target, key, color });
        res.push(target);
    }

    for (i, RenderKey { variant, key }) in FIRST_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {28 + (i * individual_width) + (i * spacing), 150 + individual_width + spacing, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyDraw { target, key, color });
        res.push(target);
    }

    for (i, RenderKey { variant, key }) in SECOND_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {36 + (i * individual_width) + (i * spacing), 150 + (individual_width + spacing) * 2, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyDraw { target, key, color });
        res.push(target);
    }

    for (i, RenderKey { variant, key }) in THIRD_ROW.iter().enumerate() {
        let i = i as u32;
        let target = rect! {44 + (i * individual_width) + (i * spacing), 150 + (individual_width + spacing) * 3, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyDraw { target, key, color });
        res.push(target);
    }

    {
        let target = rect! {44 + 2 * individual_width + 2 * spacing, 150 + (individual_width + spacing) * 4, 5 * individual_width + 5 * spacing, individual_width};
        let color = color_for_variant(SPACE_ROW.variant);
        keycode.push(KeyDraw {
            target,
            key: &SPACE_ROW.key,
            color,
        });
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

        let mut window = video_subsystem
            .window("awesome midi player", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        match window.set_opacity(0.8) {
            Ok(_) => {}
            Err(e) => info!("{:?}", e),
        }

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.present();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("./assets/Roboto/Roboto-Regular.ttf", 28)?;
        let texture_creator = canvas.texture_creator();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let keyboard_transform =
            KeyboardTransform::new(self.emitter.clone(), self.app_state.clone());

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

            {
                let app_state = self.app_state.write().unwrap();

                let surface = font
                    .render(&app_state.play_mode_label())
                    .blended(Color::RGBA(255, 255, 255, 255))
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                let TextureQuery { width, height, .. } = texture.query();
                let target = rect!(SCREEN_WIDTH - 20 - width, 20, width, height);

                canvas.copy(&texture, None, Some(target))?;
            }

            canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
            let (key_targets, key_draws) = get_keyboard_rects(SCREEN_WIDTH - 60, SCREEN_HEIGHT);
            canvas.draw_rects(&key_targets)?;

            {
                let app_state = self.app_state.read().unwrap();

                for KeyDraw { key, target, color } in key_draws {
                    canvas.set_draw_color(color);
                    canvas.fill_rect(target)?;

                    match app_state.pressed_keys.get(&key) {
                        Some(true) => {
                            canvas.set_draw_color(Color::RGBA(0, 255, 255, 255));
                            canvas.fill_rect(target)?;
                        }
                        _ => {}
                    }

                    let label = keyboard_transform.key_to_note(key.clone());

                    let color = Color::RGBA(255, 255, 255, 255);

                    let surface = font
                        .render(&label)
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
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => {
                        self.emitter.send(events::Event::Quit)?;
                        break 'running;
                    }
                    Event::KeyDown {
                        repeat: false,
                        keycode,
                        ..
                    } => {
                        let key = match keycode {
                            Some(Keycode::Space {}) => events::Key::Space,
                            Some(Keycode::Num0 {}) => events::Key::Num0,
                            Some(Keycode::Num1 {}) => events::Key::Num1,
                            Some(Keycode::Num2 {}) => events::Key::Num2,
                            Some(Keycode::Num3 {}) => events::Key::Num3,
                            Some(Keycode::Num4 {}) => events::Key::Num4,
                            Some(Keycode::Num5 {}) => events::Key::Num5,
                            Some(Keycode::Num6 {}) => events::Key::Num6,
                            Some(Keycode::Num7 {}) => events::Key::Num7,
                            Some(Keycode::Num8 {}) => events::Key::Num8,
                            Some(Keycode::Q {}) => events::Key::Q,
                            Some(Keycode::W {}) => events::Key::W,
                            Some(Keycode::E {}) => events::Key::E,
                            Some(Keycode::R {}) => events::Key::R,
                            Some(Keycode::T {}) => events::Key::T,
                            Some(Keycode::Y {}) => events::Key::Y,
                            Some(Keycode::U {}) => events::Key::U,
                            Some(Keycode::I {}) => events::Key::I,
                            Some(Keycode::O {}) => events::Key::O,
                            Some(Keycode::A {}) => events::Key::A,
                            Some(Keycode::S {}) => events::Key::S,
                            Some(Keycode::D {}) => events::Key::D,
                            Some(Keycode::F {}) => events::Key::F,
                            Some(Keycode::G {}) => events::Key::G,
                            Some(Keycode::H {}) => events::Key::H,
                            Some(Keycode::J {}) => events::Key::J,
                            Some(Keycode::K {}) => events::Key::K,
                            Some(Keycode::L {}) => events::Key::L,
                            Some(Keycode::Z {}) => events::Key::Z,
                            Some(Keycode::X {}) => events::Key::X,
                            Some(Keycode::C {}) => events::Key::C,
                            Some(Keycode::V {}) => events::Key::V,
                            Some(Keycode::B {}) => events::Key::B,
                            Some(Keycode::N {}) => events::Key::N,
                            Some(Keycode::M {}) => events::Key::M,
                            _ => events::Key::None,
                        };

                        self.emitter.send(events::Event::KeyDown(key))?;
                    }

                    Event::KeyUp {
                        repeat: false,
                        keycode,
                        ..
                    } => {
                        let key = match keycode {
                            Some(Keycode::Num0 {}) => events::Key::Num0,
                            Some(Keycode::Num1 {}) => events::Key::Num1,
                            Some(Keycode::Num2 {}) => events::Key::Num2,
                            Some(Keycode::Num3 {}) => events::Key::Num3,
                            Some(Keycode::Num4 {}) => events::Key::Num4,
                            Some(Keycode::Num5 {}) => events::Key::Num5,
                            Some(Keycode::Num6 {}) => events::Key::Num6,
                            Some(Keycode::Num7 {}) => events::Key::Num7,
                            Some(Keycode::Num8 {}) => events::Key::Num8,
                            Some(Keycode::Q {}) => events::Key::Q,
                            Some(Keycode::W {}) => events::Key::W,
                            Some(Keycode::E {}) => events::Key::E,
                            Some(Keycode::R {}) => events::Key::R,
                            Some(Keycode::T {}) => events::Key::T,
                            Some(Keycode::Y {}) => events::Key::Y,
                            Some(Keycode::U {}) => events::Key::U,
                            Some(Keycode::I {}) => events::Key::I,
                            Some(Keycode::O {}) => events::Key::O,
                            Some(Keycode::A {}) => events::Key::A,
                            Some(Keycode::S {}) => events::Key::S,
                            Some(Keycode::D {}) => events::Key::D,
                            Some(Keycode::F {}) => events::Key::F,
                            Some(Keycode::G {}) => events::Key::G,
                            Some(Keycode::H {}) => events::Key::H,
                            Some(Keycode::J {}) => events::Key::J,
                            Some(Keycode::K {}) => events::Key::K,
                            Some(Keycode::L {}) => events::Key::L,
                            Some(Keycode::Z {}) => events::Key::Z,
                            Some(Keycode::X {}) => events::Key::X,
                            Some(Keycode::C {}) => events::Key::C,
                            Some(Keycode::V {}) => events::Key::V,
                            Some(Keycode::B {}) => events::Key::B,
                            Some(Keycode::N {}) => events::Key::N,
                            Some(Keycode::M {}) => events::Key::M,
                            _ => events::Key::None,
                        };

                        self.emitter.send(events::Event::KeyUp(key))?;
                    }
                    _ => (),
                };
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        return Ok(());
    }
}
