extern crate sdl2;

use crate::app_state::*;
use crate::events;
use crate::events::EventBus;
//use crate::player::KeyboardHandler;
use super::keyboard::KeyboardRenderer;
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

        let screen_width;
        let screen_height;

        {
            screen_width = self.app_state.read().unwrap().screen_width;
            screen_height = self.app_state.read().unwrap().screen_height;
        }

        let mut window = video_subsystem
            .window("awesome midi player", screen_width, screen_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.present();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("./assets/Roboto/Roboto-Regular.ttf", 20)?;
        let texture_creator = canvas.texture_creator();

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut keyboard_renderer =
            KeyboardRenderer::new(self.emitter.clone(), self.app_state.clone());

        'running: loop {
            canvas.set_draw_color(Color::RGB(26, 22, 37));
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
                let target = rect!(screen_width - 20 - width, 20, width, height);

                canvas.copy(&texture, None, Some(target))?;
            }

            keyboard_renderer.render(&mut canvas, &texture_creator, &font)?;

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
