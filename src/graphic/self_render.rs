use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
};

use crate::{core::card, game::self_controller::SelfController};

use super::{button::ButtonColor, ui_component::Drawable, DEFAULT_FONT};

pub const CARD_SPRITESHEET: &str = "assets/cards.png";
pub const CARD_SPRITE_RATIO: f32 = SPRITE_HEIGHT as f32 / SPRITE_WIDTH as f32;
const SPRITE_WIDTH: u32 = 200;
const SPRITE_HEIGHT: u32 = 291;

pub const RAISE_COLOR: ButtonColor = ButtonColor {
    color: Color::RGB(76, 189, 45),
    hover_color: Color::RGB(52, 128, 31),
    pressed_color: Color::RGB(31, 77, 18),
    inactive_color: Color::RGB(118, 128, 115),
};

pub const CALL_COLOR: ButtonColor = ButtonColor {
    color: Color::RGB(204, 191, 47),
    hover_color: Color::RGB(128, 120, 29),
    pressed_color: Color::RGB(77, 72, 18),
    inactive_color: Color::RGB(128, 125, 102),
};

pub const FOLD_COLOR: ButtonColor = ButtonColor {
    color: Color::RGB(189, 47, 42),
    hover_color: Color::RGB(128, 31, 28),
    pressed_color: Color::RGB(77, 19, 17),
    inactive_color: Color::RGB(128, 115, 115),
};

impl Drawable for SelfController {
    fn draw(&self, gfx: &mut super::SDL2Graphics) -> Result<(), String> {
        gfx.draw_rect(self.bounds, Color::RGBA(50, 54, 49, 150))?;

        self.raise_btn.draw(gfx)?;
        self.call_btn.draw(gfx)?;
        self.fold_btn.draw(gfx)?;
        self.slider.draw(gfx)?;

        gfx.draw_rect(self.image_bounds, Color::MAGENTA)?;

        if let Some(hand) = self.state.hand {
            let creator = gfx.canvas.texture_creator();
            let tex = creator.load_texture(CARD_SPRITESHEET)?;
            let w = 180;
            let h = (w as f32 * CARD_SPRITE_RATIO) as u32;

            let c1 = rect_card_spritesheet(Some(hand.0));
            let d1 = Rect::from_center(self.bounds.center().offset(-70, -30), w, h);
            gfx.canvas.copy_ex(&tex, c1, d1, -5.0, None, false, false)?;

            let c2 = rect_card_spritesheet(Some(hand.1));
            let d2 = Rect::from_center(self.bounds.center().offset(70, -30), w, h);
            gfx.canvas.copy_ex(&tex, c2, d2, 5.0, None, false, false)?;
        }

        let mut f = DEFAULT_FONT;
        f.size = 36;
        if !self.state.name.is_empty() {
            gfx.draw_string(
                &self.state.name,
                f,
                Point::new(250, self.bounds.y + 50),
                false,
            );
        }

        f.size = 24;
        let t = format!("Actual money: {}€", self.state.cash);
        gfx.draw_string(&t, f, Point::new(250, self.bounds.y + 100), false);

        let t = format!("Actual bet: {}€", self.state.bet);
        gfx.draw_string(&t, f, Point::new(250, self.bounds.y + 130), false);

        let t = format!("Raise {}€ more", self.to_raise());
        gfx.draw_string(&t, f, Point::new(250, self.bounds.y + 160), false);

        Ok(())
    }
}

fn rect_card_spritesheet(card: Option<card::Card>) -> Rect {
    match card {
        Some(card) => {
            let x_offset = {
                if card.value == card::Value::Ace {
                    0
                } else {
                    card.value as i32 + 1
                }
            };
            let y_offset = card.suit as i32;

            Rect::new(
                (x_offset % 13) * SPRITE_WIDTH as i32,
                y_offset * SPRITE_HEIGHT as i32,
                SPRITE_WIDTH as u32,
                SPRITE_HEIGHT as u32,
            )
        }
        None => Rect::new(
            2 * SPRITE_WIDTH as i32,
            4 * SPRITE_HEIGHT as i32,
            SPRITE_WIDTH,
            SPRITE_HEIGHT,
        ),
    }
}
