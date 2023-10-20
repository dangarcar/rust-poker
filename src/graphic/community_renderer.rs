use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::core::card::Card;

use crate::game::game_render::{rect_card_spritesheet, CARD_SPRITE_RATIO};

use super::font::DEFAULT_FONT;
use super::{ui_component::Drawable, HEIGHT, WIDTH};

#[derive(Default)]
pub struct CommunityRenderer {
    cards: [Option<Card>; 5],
    pub pot: i32,
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
                        WIDTH as i32 / 2 - 2 * (w + 15) + i as i32 * (w + 15),
                        HEIGHT as i32 / 2 - 40,
                    ),
                    w as u32,
                    h,
                );
                gfx.canvas.copy(tex, src, dst)?;
            }
        }

        let pot_center = Point::new(WIDTH as i32/2, HEIGHT as i32/2 + 100);
        gfx.draw_rect(Rect::from_center(pot_center, 300, 100), Color::RGB(212, 175, 55))?;
        gfx.draw_string(&format!("{}€",self.pot), DEFAULT_FONT.derive_size(36), pot_center, true)?;

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
