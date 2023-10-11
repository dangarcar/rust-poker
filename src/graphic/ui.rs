use sdl2::event::Event;

use crate::game::self_controller::SelfController;

use super::{
    ui_component::{Drawable, EventReceiver},
    SDL2Graphics,
};

pub struct UI {
    btn: SelfController,
}

impl UI {
    pub fn new() -> Self {
        UI {
            btn: SelfController::default(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        if let Some(s) = self.btn.handle_event(event) {
            println!("{s:?}");
        }
        Ok(())
    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.btn.draw(gfx)?;
        Ok(())
    }
}
