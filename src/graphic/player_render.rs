use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use crate::game::{player_state::PlayerState, game_render::{rect_card_spritesheet, CARD_SPRITE_RATIO}};

use super::{font::DEFAULT_FONT, ui_component::Drawable};

pub struct PlayerRenderer {
    bounds: Rect,
    image_bounds: Rect,
    state: PlayerState,
}

impl PlayerRenderer {
    pub fn new(p: Point, state: PlayerState) -> Self {
        let bounds = Rect::from_center(p, 260, 140);
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

    fn draw_hand(&self, gfx: &mut super::SDL2Graphics<'_>) -> Result<(), String> {
        Ok(if let Some(tex) = gfx.tex_cache.get("CARD") {
            let w = 52;
            let h = (w as f32 * CARD_SPRITE_RATIO) as i32;

            let p = self.bounds.bottom_right().offset(-w - 10, -h - 10);
            gfx.canvas.copy(
                tex,
                rect_card_spritesheet(self.state.hand.map(|hand| hand.0)),
                Rect::new(p.x, p.y, w as u32, h as u32),
            )?;

            let p = p.offset(-w - 10, 0);
            gfx.canvas.copy(
                tex,
                rect_card_spritesheet(self.state.hand.map(|hand| hand.1)),
                Rect::new(p.x, p.y, w as u32, h as u32),
            )?;
        })
    }
}

impl Drawable for PlayerRenderer {
    fn draw(&self, gfx: &mut super::SDL2Graphics) -> Result<(), String> {
        gfx.draw_rect(self.bounds, Color::GRAY)?;
        gfx.draw_rect(self.image_bounds, Color::MAGENTA)?;

        gfx.draw_string(
            &self.state.name,
            DEFAULT_FONT,
            Point::new(self.image_bounds.right() + 10, self.image_bounds.y + 2),
            false,
        )?;

        gfx.draw_string(
            &format!("{}€", self.state.cash),
            DEFAULT_FONT,
            Point::new(self.image_bounds.x, self.image_bounds.bottom() + 10),
            false,
        )?;

        gfx.draw_string(
            &format!("{}€", self.state.bet),
            DEFAULT_FONT.derive_color(Color::GREEN),
            Point::new(self.image_bounds.x, self.image_bounds.bottom() + 40),
            false,
        )?;

        self.draw_hand(gfx)?;

        if self.state.folded {
            gfx.draw_rect(self.bounds, Color::RGBA(0, 0, 0, 180))?;
        } else if self.state.turn {
            gfx.draw_rect(self.bounds, Color::RGBA(255, 255, 0, 100))?;
        }

        Ok(())
    }
}
