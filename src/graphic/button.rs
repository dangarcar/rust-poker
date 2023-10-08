use sdl2::{rect::Rect, pixels::Color, event::Event, mouse::MouseButton};

use super::{ui_component::{EventReceiver, Drawable}, SDL2Graphics, font::FontParams, DEFAULT_FONT};

use sdl2::gfx::primitives::*;

pub struct Button {
    text: String,
    is_hovered: bool,
    bounds: Rect,
    color: Color,
    hover_color: Color,
    font_params: FontParams,
}

impl EventReceiver for Button {
    fn handle_event(&mut self, event: &sdl2::event::Event) {
        match event {
            Event::MouseMotion {x, y, ..} => {
                self.is_hovered = self.bounds.contains_point((*x, *y));
            }
            Event::MouseButtonDown {mouse_btn, x, y, .. } => {
                if *mouse_btn == MouseButton::Left && self.bounds.contains_point((*x, *y)) {
                    println!("Button pressed");
                }
            }
            _ => {}
        }
    }
}

impl Drawable for Button {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        if self.is_hovered {
            gfx.canvas.set_draw_color(self.hover_color);
        }
        else {
            gfx.canvas.set_draw_color(self.color);
        }

        let r = self.bounds;
        gfx.canvas.rounded_box(r.x as i16, r.y as i16, r.x as i16+r.w as i16, r.y as i16+r.h as i16, 34, gfx.canvas.draw_color())?;

        gfx.draw_string(&self.text, self.font_params, self.bounds.center(), true);

        Ok(())
    }
}

impl Button {
    pub fn new(text: String, bounds: Rect, color: Color, hover_color: Color) -> Self {
        Button { 
            text, 
            is_hovered: false, 
            bounds, color, 
            hover_color, 
            font_params: DEFAULT_FONT ,
        }
    }
}