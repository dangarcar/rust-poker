use sdl2::event::Event;

use crate::graphic::SDL2Graphics;
use crate::graphic::ui;

pub struct Game {
    ui: ui::UI,
}

impl Game {
    pub fn new() -> Self {
        Game { 
            ui: ui::UI::new()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {

    }

    pub fn update(&mut self) {

    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.ui.render(gfx)?;
        Ok(())
    }
}