use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use crate::game::player_state::PlayerState;

use super::{ui_component::Drawable, DEFAULT_FONT};

pub struct PlayerRenderer {
    bounds: Rect,
    image_bounds: Rect,
    state: PlayerState,
}

impl PlayerRenderer {
    pub fn new(p: Point, state: PlayerState) -> Self {
        let bounds = Rect::from_center(p, 160, 100);
        let image_bounds = Rect::new(bounds.x + 10, bounds.y + 10, 50, 50);

        PlayerRenderer {
            bounds,
            image_bounds,
            state,
        }
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }
}

impl Drawable for PlayerRenderer {
    fn draw(&self, gfx: &mut super::SDL2Graphics) -> Result<(), String> {
        gfx.draw_rect(self.bounds, Color::GRAY)?;
        gfx.draw_rect(self.image_bounds, Color::MAGENTA)?;

        let t = format!("{}€", self.state.cash);
        gfx.draw_string(
            &t,
            DEFAULT_FONT,
            Point::new(self.image_bounds.right() + 10, self.image_bounds.y),
            false,
        );

        gfx.draw_string(
            &self.state.name,
            DEFAULT_FONT,
            Point::new(self.image_bounds.x, self.image_bounds.bottom() + 10),
            false,
        );

        Ok(())
    }
}
