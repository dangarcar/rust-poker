use sdl2::{rect::{Rect, Point}, pixels::Color, ttf::FontStyle, event::Event, mouse::MouseButton};

use super::{event_receiver::EventReceiver, SDL2Graphics, WIDTH, HEIGHT};

use sdl2::gfx::primitives::*;

pub struct Button {
    text: String,
    is_hovered: bool,
    bounds: Rect,
    color: Color,
    hover_color: Color,
}

impl EventReceiver for Button {
    fn handle_event(&mut self, event: &sdl2::event::Event) {
        match event {
            Event::MouseMotion {x, y, ..} => {
                self.is_hovered = self.bounds.contains_point((*x, *y));
            }
            Event::MouseButtonDown {mouse_btn, x, y, .. } => {
                match mouse_btn {
                    MouseButton::Left => {
                        if self.bounds.contains_point((*x, *y)) {
                            println!("Button pressed");
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Button {
    pub fn default() -> Self {
        Button { text: "Hello world".to_string(), is_hovered: false, bounds: Rect::new(WIDTH as i32/2 - 100, 4*HEIGHT as i32/5, 200, 200), color: Color::MAGENTA, hover_color: Color::BLUE }
    }

    pub fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        if self.is_hovered {
            gfx.canvas.set_draw_color(self.hover_color);
        }
        else {
            gfx.canvas.set_draw_color(self.color);
        }

        let r = self.bounds;
        gfx.canvas.rounded_box(r.x as i16, r.y as i16, r.x as i16+r.w as i16, r.y as i16+r.h as i16, 34, gfx.canvas.draw_color())?;

        gfx.draw_string(&self.text, super::font::FontParams::new(49, FontStyle::NORMAL, Color::WHITE), Point::new(self.bounds.x,self.bounds.y), false);

        Ok(())
    }
}