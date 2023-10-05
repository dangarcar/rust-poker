pub mod core;

pub mod graphic;

pub mod music;

pub mod game;


extern crate sdl2;

use crate::core::deck::Deck;
use graphic::renderer::CARD_SPRITE_RATIO;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

fn _draw(canvas: &mut WindowCanvas, texture: &Texture) {
    let mut deck = Deck::default();

    let mut x = 100;
    let mut y = 100;

    if deck.len() > 0 {
        for _ in 0..deck.len() - 1 {
            x -= 1;
            y -= 1;
            let dst = Rect::new(x, y, 200, (200 as f32 * CARD_SPRITE_RATIO) as u32);
            graphic::renderer::render_card(canvas, texture, &None, dst).unwrap();
        }

        let card = deck.take();
        let dst = Rect::new(500, 100, 200, (200 as f32 * CARD_SPRITE_RATIO) as u32);
        graphic::renderer::render_card(canvas, texture, &card, dst).unwrap();
    }
}