use sdl2::{render::{self, TextureQuery}, rect::{Rect, Point}, ttf::Font, pixels::Color};
use crate::core::card;

pub const CARD_SPRITESHEET: &str = "assets/cards.png";
pub const CARD_SPRITE_RATIO: f32 = SPRITE_HEIGHT as f32/SPRITE_WIDTH as f32;
const SPRITE_WIDTH: i32 = 200;
const SPRITE_HEIGHT: i32 = 291;

pub fn render_card(canvas: &mut render::WindowCanvas, texture: &render::Texture, card:&Option<card::Card>, dst: Rect) -> Result<(), String> {
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
                let src = Rect::new(
                (x_offset % 13) * SPRITE_WIDTH, 
                y_offset * SPRITE_HEIGHT, 
                SPRITE_WIDTH as u32, 
                SPRITE_HEIGHT as u32);
            
            canvas.copy(&texture, src, dst)?;
        }
        None => {
            let src = Rect::new(
                2 * SPRITE_WIDTH, 
                4 * SPRITE_HEIGHT, 
                SPRITE_WIDTH as u32, 
                SPRITE_HEIGHT as u32);
            
            canvas.copy(&texture, src, dst)?;
        }
    }

    Ok(())
}

pub fn render_text(canvas: &mut render::WindowCanvas, font: &Font, txt: &str, color: Color, bounds: Rect) {
    let creator = canvas.texture_creator();
    let surf = font.render(txt).blended(color).unwrap();
    let txt_text = creator.create_texture_from_surface(surf).unwrap();
    canvas.copy(&txt_text, None, bounds).unwrap();
}

pub fn render_text_centered(canvas: &mut render::WindowCanvas, font: &Font, txt: &str, color: Color, c: Point) {
    let creator = canvas.texture_creator();
    let surf = font.render(txt).blended(color).unwrap();
    let txt_text = creator.create_texture_from_surface(surf).unwrap();
    let TextureQuery{width, height, ..} = txt_text.query();
    canvas.copy(&txt_text, None, Rect::from_center(c, width, height)).unwrap();
}

pub fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (super::WIDTH as i32 - w) / 2;
    let cy = (super::HEIGHT as i32 - h) / 2;
    Rect::new(cx, cy, w as u32, h as u32)
}