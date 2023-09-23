use sdl2::{render, rect};
use crate::core::card;

pub const CARD_SPRITESHEET: &str = "assets/cards.png";
pub const CARD_SPRITE_RATIO: f32 = SPRITE_HEIGHT as f32/SPRITE_WIDTH as f32;
const SPRITE_WIDTH: i32 = 200;
const SPRITE_HEIGHT: i32 = 291;

pub fn render_card(canvas: &mut render::WindowCanvas, texture: &render::Texture, card:&Option<card::Card>, dst: rect::Rect) -> Result<(), String> {
    match card {
        Some(card) => {
            let x_offset = {
                if card.value == card::Value::Ace {
                    0
                } else {
                    card.value as i32+1
                }
            };
            let y_offset = card.suit as i32;
            let src = rect::Rect::new(
                (x_offset % 13) * SPRITE_WIDTH, 
                y_offset * SPRITE_HEIGHT, 
                SPRITE_WIDTH as u32, 
                SPRITE_HEIGHT as u32);
            
            canvas.copy(&texture, src, dst)?;
        }
        None => {
            let src = rect::Rect::new(
                2 * SPRITE_WIDTH, 
                4 * SPRITE_HEIGHT, 
                SPRITE_WIDTH as u32, 
                SPRITE_HEIGHT as u32);
            
            canvas.copy(&texture, src, dst)?;
        }
    }

    Ok(())
}