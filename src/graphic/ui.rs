use sdl2::{event::Event, rect::Rect, pixels::Color};

use super::{SDL2Graphics, button::Button, ui_component::{EventReceiver, Drawable}, WIDTH, HEIGHT, slider::Slider};


pub struct UI {
    btn : Button,
    sld: Slider
}

impl UI {
    pub fn new() -> Self {
        UI { 
            btn : Button::new("Hello world".to_string(),Rect::new(WIDTH as i32/2 - 100, 4*HEIGHT as i32/5, 200, 200), Color::MAGENTA,Color::BLUE),
            sld: Slider::new(Rect::new(WIDTH as i32/2 + 300, 4*HEIGHT as i32/5, 200, 40), 10, 100, Color::GREEN, Color::GRAY, Color::RED),
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        self.btn.handle_event(event);
        self.sld.handle_event(event);
        Ok(())
    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.btn.draw(gfx)?;
        self.sld.draw(gfx)?;
        Ok(())
    }
}