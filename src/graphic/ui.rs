use super::{SDL2Graphics, button::Button};


pub struct UI {
    btn : Button,
}

impl UI {
    pub fn new() -> Self {
        UI { 
            btn: Button::default(),
        }
    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.btn.draw(gfx)?;
        Ok(())
    }
}