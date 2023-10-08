use std::cmp;

use sdl2::{rect::{Rect, Point}, mouse::MouseButton, event::Event, pixels::Color};

use super::{ui_component::{EventReceiver, Drawable}, DEFAULT_FONT};

pub struct Slider {
    bounds: Rect,
    min_value: i32,
    max_value: i32,
    current: f32,
    color: Color,
    slider_color: Color,
    slided_color: Color,
    show_value: bool,
}

impl EventReceiver for Slider {
    fn handle_event(&mut self, event: &sdl2::event::Event) {
        match event {
            Event::MouseMotion { mousestate, x, y, .. } => {
                if self.bounds.contains_point((*x, *y)) && mousestate.is_mouse_button_pressed(MouseButton::Left) {
                    self.current = (x - self.bounds.x - self.bounds.h/2) as f32 / (self.bounds.w - self.bounds.h) as f32;
                    self.current = f32::max(0.0, self.current);
                    self.current = f32::min(1.0, self.current);
                }
            }
            Event::MouseButtonDown {mouse_btn, x, y, .. } => {
                if *mouse_btn == MouseButton::Left && self.bounds.contains_point((*x, *y)) {
                    self.current = (x - self.bounds.x - self.bounds.h/2) as f32 / (self.bounds.w - self.bounds.h) as f32;
                    self.current = f32::max(0.0, self.current);
                    self.current = f32::min(1.0, self.current);
                }
            }
            _ => {}
        }
    }
}

impl Drawable for Slider {
    fn draw(&self, gfx: &mut super::SDL2Graphics) -> Result<(), String> {
        gfx.canvas.set_draw_color(self.slided_color);
        gfx.canvas.fill_rect(self.bounds)?;
        
        gfx.canvas.set_draw_color(self.slider_color);
        let h = self.bounds.h;
        let x = self.bounds.x + (self.bounds.w as f32 * self.current) as i32 - h/2;
        let x = cmp::max(self.bounds.x, x);
        let x = cmp::min(self.bounds.x+self.bounds.w-h, x);
        let r = Rect::new(x, self.bounds.y, h as u32, h as u32);
        gfx.canvas.fill_rect(r)?;

        gfx.canvas.set_draw_color(self.color);
        gfx.canvas.fill_rect(Rect::new(self.bounds.x, self.bounds.y, x as u32-self.bounds.x as u32, self.bounds.height()))?;

        if self.show_value {
            gfx.draw_string(&self.value().to_string(), DEFAULT_FONT, Point::new(self.bounds.x+self.bounds.w, self.bounds.y), false)
        }

        Ok(())
    }
}

impl Slider {
    pub fn new(bounds: Rect, min_value: i32, max_value: i32, color: Color, slider_color: Color, slided_color: Color) -> Self {
        Slider {
            bounds, 
            min_value, 
            max_value, 
            current: 0.0,
            color,
            slider_color,
            slided_color,
            show_value: true,
        }
    }

    pub fn value(&self) -> i32 {
        self.min_value + ((self.max_value-self.min_value) as f32 * self.current) as i32
    }
}