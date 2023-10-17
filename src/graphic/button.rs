use sdl2::{event::Event, mouse::MouseButton, pixels::Color, rect::Rect};

use super::{
    font::{FontParams, DEFAULT_FONT},
    ui_component::{Drawable, EventReceiver},
    SDL2Graphics,
};

#[derive(Debug, Clone, Copy)]
pub struct ButtonColor {
    pub color: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub inactive_color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Inactive,
}

pub struct Button {
    text: String,
    bounds: Rect,
    color: ButtonColor,
    font_params: FontParams,

    state: ButtonState,
}

impl EventReceiver<ButtonState> for Button {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> ButtonState {
        match event {
            Event::MouseMotion { x, y, .. } => {
                if self.bounds.contains_point((*x, *y)) {
                    self.state = ButtonState::Hovered;
                } else {
                    self.state = ButtonState::Normal;
                }
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if *mouse_btn == MouseButton::Left && self.bounds.contains_point((*x, *y)) {
                    self.state = ButtonState::Pressed;
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if *mouse_btn == MouseButton::Left && self.bounds.contains_point((*x, *y)) {
                    self.state = ButtonState::Hovered;
                }
            }
            _ => {
                self.state = ButtonState::Normal;
            }
        }

        self.state
    }
}

impl Drawable for Button {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        match self.state {
            ButtonState::Inactive => gfx.canvas.set_draw_color(self.color.inactive_color),
            ButtonState::Normal => gfx.canvas.set_draw_color(self.color.color),
            ButtonState::Hovered => gfx.canvas.set_draw_color(self.color.hover_color),
            ButtonState::Pressed => gfx.canvas.set_draw_color(self.color.pressed_color),
        }

        gfx.canvas.fill_rect(self.bounds)?;

        const W: i32 = 20;

        gfx.canvas.set_draw_color(Color::RGBA(255, 255, 255, 50));
        gfx.canvas.fill_rect(Rect::new(
            self.bounds.left(),
            self.bounds.top(),
            W as u32,
            self.bounds.height(),
        ))?;
        gfx.canvas.fill_rect(Rect::new(
            self.bounds.left(),
            self.bounds.top(),
            self.bounds.width(),
            W as u32,
        ))?;

        gfx.canvas.set_draw_color(Color::RGBA(0, 0, 0, 50));
        gfx.canvas.fill_rect(Rect::new(
            self.bounds.right() - W,
            self.bounds.y,
            W as u32,
            self.bounds.height(),
        ))?;
        gfx.canvas.fill_rect(Rect::new(
            self.bounds.left(),
            self.bounds.bottom() - W,
            self.bounds.width(),
            W as u32,
        ))?;

        gfx.draw_string(&self.text, self.font_params, self.bounds.center(), true);

        Ok(())
    }
}

impl Button {
    pub fn new(text: String, bounds: Rect, color: ButtonColor) -> Self {
        Button {
            text,
            bounds,
            color,

            font_params: DEFAULT_FONT,
            state: ButtonState::Normal,
        }
    }

    pub fn set_font(&mut self, f: FontParams) {
        self.font_params = f;
    }

    pub fn set_inactive(&mut self) {
        self.state = ButtonState::Inactive;
    }

    pub fn set_active(&mut self) {
        if self.state == ButtonState::Inactive {
            self.state = ButtonState::Normal;
        }
    }
}
