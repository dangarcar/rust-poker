use sdl2::event::Event;

use super::{SDL2Graphics, button::Button, event_receiver::EventReceiver};


pub struct UI {
    btn : Button,
}

impl UI {
    pub fn new() -> Self {
        UI { 
            btn: Button::default(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        self.btn.handle_event(event);
        Ok(())
    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.btn.draw(gfx)?;
        Ok(())
    }
}