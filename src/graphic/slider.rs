use sdl2::{
    event::Event,
    mouse::MouseButton,
    pixels::Color,
    rect::{Point, Rect},
};

use super::ui_component::{Drawable, EventReceiver};

pub struct Slider {
    bounds: Rect,
    slide_bounds: Rect,
    current: f32,
    color: Color,
    slider_color: Color,
    slided_color: Color,
}

impl EventReceiver<f32> for Slider {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> f32 {
        match event {
            Event::MouseMotion {
                mousestate, x, y, ..
            } => {
                if self.bounds.contains_point((*x, *y))
                    && mousestate.is_mouse_button_pressed(MouseButton::Left)
                {
                    self.calc_value(*x);
                }
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if *mouse_btn == MouseButton::Left && self.bounds.contains_point((*x, *y)) {
                    self.calc_value(*x);
                }
            }
            _ => {}
        }

        self.current
    }
}

impl Drawable for Slider {
    fn draw(&self, gfx: &mut super::SDL2Graphics) -> Result<(), String> {
        gfx.canvas.set_draw_color(self.slided_color);
        gfx.canvas.fill_rect(self.bounds)?;

        gfx.canvas.set_draw_color(self.color);
        gfx.canvas.fill_rect(Rect::new(
            self.bounds.x,
            self.bounds.y,
            (self.slide_bounds.w as f32 * self.current) as u32,
            self.bounds.height(),
        ))?;

        gfx.canvas.set_draw_color(self.slider_color);
        let h = self.bounds.h;
        let r = Rect::from_center(
            Point::new(
                self.slide_bounds.x + (self.slide_bounds.w as f32 * self.current) as i32,
                self.slide_bounds.y + h / 2,
            ),
            h as u32,
            h as u32,
        );
        gfx.canvas.fill_rect(r)?;

        Ok(())
    }
}

impl Slider {
    pub fn new(bounds: Rect, color: Color, slider_color: Color, slided_color: Color) -> Self {
        Slider {
            bounds,
            slide_bounds: Rect::new(
                bounds.x + bounds.h / 2,
                bounds.y,
                bounds.width() - bounds.height(),
                bounds.height(),
            ),
            current: 0.0,
            color,
            slider_color,
            slided_color,
        }
    }

    pub fn value(&self) -> f32 {
        self.current
    }

    fn calc_value(&mut self, x: i32) {
        self.current = (x - self.slide_bounds.x) as f32 / self.slide_bounds.width() as f32;
        self.current = f32::max(0.0, self.current);
        self.current = f32::min(1.0, self.current);
    }
}
