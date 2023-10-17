use sdl2::rect::{Point, Rect};

use crate::core::card::Card;

use super::{
    self_render::{rect_card_spritesheet, CARD_SPRITE_RATIO},
    ui_component::Drawable,
    HEIGHT, WIDTH,
};

#[derive(Default)]
pub struct CommunityRenderer {
    cards: [Option<Card>; 5],
}

impl Drawable for CommunityRenderer {
    fn draw(&self, gfx: &mut super::SDL2Graphics) -> Result<(), String> {
        if let Some(tex) = gfx.tex_cache.get("CARD") {
            let w = 100;
            let h = (w as f32 * CARD_SPRITE_RATIO) as u32;

            for (i, c) in self.cards.iter().enumerate() {
                let src = rect_card_spritesheet(*c);
                let dst = Rect::from_center(
                    Point::new(
                        WIDTH as i32 / 2 - 2 * (w + 20) + i as i32 * (w + 20),
                        HEIGHT as i32 / 2,
                    ),
                    w as u32,
                    h,
                );
                gfx.canvas.copy(tex, src, dst)?;
            }
        }

        Ok(())
    }
}

impl CommunityRenderer {
    pub fn add_card(&mut self, card: Card) {
        for o in self.cards.iter_mut() {
            if o.is_none() {
                *o = Some(card);
                return;
            }
        }
    }
}
