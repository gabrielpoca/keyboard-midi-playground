use crate::app_state::*;
use crate::events;
use crate::player::KeyboardHandler;
use crossbeam_channel::Sender;
use sdl2::pixels::*;
use sdl2::rect::*;
use sdl2::render::TextureQuery;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::borrow::Cow;
use std::error::Error;
use std::sync::*;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

struct KeyboardKey<'a> {
    key: events::Key,
    variant: u32,
    label: Option<&'a str>,
}

struct KeyToDraw<'a> {
    target: Rect,
    label: Cow<'a, str>,
    key: &'a events::Key,
    color: Color,
}

static ROW_LENGTH: u32 = 10;

static NUM_ROW: [KeyboardKey; 10] = [
    KeyboardKey {
        label: Some("min"),
        variant: 1,
        key: events::Key::Num1,
    },
    KeyboardKey {
        label: Some("h min"),
        variant: 1,
        key: events::Key::Num2,
    },
    KeyboardKey {
        variant: 1,
        label: Some("m min"),
        key: events::Key::Num3,
    },
    KeyboardKey {
        label: None,
        variant: 2,
        key: events::Key::Num4,
    },
    KeyboardKey {
        label: None,
        variant: 2,
        key: events::Key::Num5,
    },
    KeyboardKey {
        label: None,
        variant: 2,
        key: events::Key::Num6,
    },
    KeyboardKey {
        label: None,
        variant: 2,
        key: events::Key::Num7,
    },
    KeyboardKey {
        variant: 2,
        label: None,
        key: events::Key::Num8,
    },
    KeyboardKey {
        variant: 2,
        label: None,
        key: events::Key::Num9,
    },
    KeyboardKey {
        variant: 2,
        label: None,
        key: events::Key::Num0,
    },
];

static FIRST_ROW: [KeyboardKey; 10] = [
    KeyboardKey {
        variant: 1,
        label: None,
        key: events::Key::Q,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::W,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::E,
    },
    KeyboardKey {
        variant: 0,
        key: events::Key::R,
        label: None,
    },
    KeyboardKey {
        variant: 0,
        key: events::Key::T,
        label: None,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::Y,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::U,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::I,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::O,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::P,
    },
];

static SECOND_ROW: [KeyboardKey; 9] = [
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::A,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::S,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::D,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::F,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::G,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::H,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::J,
    },
    KeyboardKey {
        label: None,
        variant: 0,
        key: events::Key::K,
    },
    KeyboardKey {
        variant: 0,
        label: None,
        key: events::Key::L,
    },
];

static THIRD_ROW: [KeyboardKey; 7] = [
    KeyboardKey {
        label: Some("-1 o"),
        variant: 1,
        key: events::Key::Z,
    },
    KeyboardKey {
        variant: 1,
        label: Some("+1 o"),
        key: events::Key::X,
    },
    KeyboardKey {
        label: Some("-1 s"),
        variant: 1,
        key: events::Key::C,
    },
    KeyboardKey {
        label: Some("+1 s"),
        variant: 1,
        key: events::Key::V,
    },
    KeyboardKey {
        label: None,
        variant: 2,
        key: events::Key::B,
    },
    KeyboardKey {
        label: None,
        variant: 2,
        key: events::Key::N,
    },
    KeyboardKey {
        variant: 2,
        label: None,
        key: events::Key::M,
    },
];

static SPACE_ROW: KeyboardKey = KeyboardKey {
    label: Some("Mode"),
    variant: 1,
    key: events::Key::Space,
};

fn color_for_variant(variant: u32) -> Color {
    return match variant {
        0 => Color::RGBA(91, 72, 125, 0),
        1 => Color::RGBA(53, 37, 69, 255),
        _ => Color::RGBA(26, 22, 37, 255),
    };
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

fn get_keyboard_rects<'a>(
    keyboard_handler: &'a KeyboardHandler,
    cons_width: u32,
    _cons_height: u32,
) -> (Vec<Rect>, Vec<KeyToDraw<'a>>) {
    let spacing = 10;
    let total_spacing = 8 * spacing;
    let individual_width = (cons_width - total_spacing) / ROW_LENGTH;

    let mut res = Vec::new();
    let mut keycode = Vec::new();

    for (
        i,
        KeyboardKey {
            variant,
            key,
            label,
        },
    ) in NUM_ROW.iter().enumerate()
    {
        let key_label;

        match label {
            Some(actual_label) => {
                key_label = Cow::from(*actual_label);
            }
            None => {
                key_label = Cow::from(keyboard_handler.key_to_note(key.clone()));
            }
        }

        let i = i as u32;
        let target = rect! {20 + (i * individual_width) + (i * spacing), 150, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyToDraw {
            label: key_label,
            target,
            key,
            color,
        });
        res.push(target);
    }

    for (
        i,
        KeyboardKey {
            variant,
            key,
            label,
        },
    ) in FIRST_ROW.iter().enumerate()
    {
        let key_label;

        match label {
            Some(actual_label) => {
                key_label = Cow::from(*actual_label);
            }
            None => {
                key_label = Cow::from(keyboard_handler.key_to_note(key.clone()));
            }
        }

        let i = i as u32;
        let target = rect! {28 + (i * individual_width) + (i * spacing), 150 + individual_width + spacing, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyToDraw {
            label: key_label,
            target,
            key,
            color,
        });
        res.push(target);
    }

    for (
        i,
        KeyboardKey {
            variant,
            key,
            label,
        },
    ) in SECOND_ROW.iter().enumerate()
    {
        let key_label;

        match label {
            Some(actual_label) => {
                key_label = Cow::from(*actual_label);
            }
            None => {
                key_label = Cow::from(keyboard_handler.key_to_note(key.clone()));
            }
        }

        let i = i as u32;
        let target = rect! {36 + (i * individual_width) + (i * spacing), 150 + (individual_width + spacing) * 2, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyToDraw {
            label: key_label,
            target,
            key,
            color,
        });
        res.push(target);
    }

    for (
        i,
        KeyboardKey {
            variant,
            key,
            label,
        },
    ) in THIRD_ROW.iter().enumerate()
    {
        let key_label;

        match label {
            Some(actual_label) => {
                key_label = Cow::from(*actual_label);
            }
            None => {
                key_label = Cow::from(keyboard_handler.key_to_note(key.clone()));
            }
        }

        let i = i as u32;
        let target = rect! {44 + (i * individual_width) + (i * spacing), 150 + (individual_width + spacing) * 3, individual_width, individual_width};
        let color = color_for_variant(*variant);
        keycode.push(KeyToDraw {
            label: key_label,
            target,
            key,
            color,
        });
        res.push(target);
    }

    {
        let target = rect! {44 + 2 * individual_width + 2 * spacing, 150 + (individual_width + spacing) * 4, 5 * individual_width + 5 * spacing, individual_width};
        let color = color_for_variant(SPACE_ROW.variant);
        keycode.push(KeyToDraw {
            label: Cow::from(SPACE_ROW.label.unwrap()),
            target,
            key: &SPACE_ROW.key,
            color,
        });
        res.push(target);
    }

    return (res, keycode);
}

pub struct KeyboardRenderer {
    keyboard_handler: KeyboardHandler,
    app_state: Arc<RwLock<AppState>>,
}

impl<'l> KeyboardRenderer {
    pub fn new(emitter: Sender<events::Event>, app_state: Arc<RwLock<AppState>>) -> Self {
        let app_state_clone = app_state.clone();

        return Self {
            app_state,
            keyboard_handler: KeyboardHandler::new(emitter, app_state_clone),
        };
    }

    pub fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font<'l, 'static>,
    ) -> Result<(), Box<dyn Error>> {
        let app_state = self.app_state.read().unwrap();

        let (_key_targets, key_draws) = get_keyboard_rects(
            &self.keyboard_handler,
            app_state.screen_width - 60,
            app_state.screen_height,
        );

        for KeyToDraw {
            key,
            target,
            color,
            label,
        } in key_draws
        {
            canvas.set_draw_color(color);
            canvas.fill_rect(target)?;

            match app_state.pressed_keys.get(&key) {
                Some(true) => {
                    canvas.set_draw_color(Color::RGBA(171, 136, 213, 255));
                    canvas.fill_rect(target)?;
                }
                _ => {}
            }

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

        //canvas.set_draw_color(Color::RGBA(0, 255, 255, 255));
        //canvas.draw_rects(&key_targets)?;

        return Ok(());
    }
}
