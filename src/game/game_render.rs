use sdl2::{pixels::Color, rect::{Rect, Point}};

use crate::{game::{Game, GamePhase}, core::card::{Card, Value}, graphic::{ui_component::Drawable, SDL2Graphics, font::DEFAULT_FONT, WIDTH, START_DELAY, HEIGHT}};

use super::player_state::PlayerState;

pub const CARD_SPRITE_RATIO: f32 = SPRITE_HEIGHT as f32 / SPRITE_WIDTH as f32;
const SPRITE_WIDTH: u32 = 200;
const SPRITE_HEIGHT: u32 = 291;

impl Drawable for Game {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        //CLEAR SCREEN
        gfx.clear()?;

        self.ui.draw(gfx)?;

        match self.phase {
            GamePhase::Start => {
                if let Some(bg) = gfx.tex_cache.get("TITLE") {
                    gfx.canvas.copy(bg, None, None)?;
                }
                gfx.draw_rect(
                    Rect::new(0, 0, WIDTH, HEIGHT), 
                    Color::RGBA(0, 0, 0, (255.0 * (START_DELAY - self.delay).as_secs_f32()/START_DELAY.as_secs_f32()) as u8)
                )?;
            },
            GamePhase::Pause => {
                gfx.draw_rect(Rect::new(0, 0, WIDTH, HEIGHT), Color::RGBA(0, 0, 0, 200))?;
                gfx.draw_string(
                    "PAUSED",
                    DEFAULT_FONT.derive_size(128),
                    Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2),
                    true,
                )?;
            },
            GamePhase::Ended(rank, i, pot) => {
                gfx.draw_rect(Rect::new(0, 0, WIDTH, HEIGHT), Color::RGBA(0, 0, 0, 200))?;

                if let Some(players) = &self.players {
                    let big_color; let big_txt; let small_txt;
                    if i != self.myself {
                        big_color = Color::RED;
                        big_txt = "GAME OVER".to_string();
                        small_txt = format!("Player \"{}\" won {}€", players[i].name, pot);
                    } else {
                        big_color = Color::GREEN;
                        big_txt = "YOU WON".to_string();
                        small_txt = format!("You have won {}€", pot);
                    }

                    gfx.draw_string(
                        &big_txt,
                        DEFAULT_FONT.derive_size(128).derive_color(big_color),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 - 100),
                        true,
                    )?;
                    gfx.draw_string(
                        &small_txt,
                        DEFAULT_FONT.derive_size(48),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2),
                        true,
                    )?;
                    gfx.draw_string(
                        &format!("Rank: {:?}", rank),
                        DEFAULT_FONT.derive_size(48),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 + 70),
                        true,
                    )?;
                    draw_hand(gfx, players, i, Point::new(WIDTH as i32/2, HEIGHT as i32/2 + 280))?;
                }
            }
            GamePhase::Showdown(i) => {
                if let Some(players) = &self.players {
                    gfx.draw_rect(Rect::new(0, 0, WIDTH, HEIGHT), Color::RGBA(0, 0, 0, 200))?;
                    gfx.draw_string(
                        &format!("Player \"{}\" has {:?}", players[i].name, players[i].rank.unwrap()),
                        DEFAULT_FONT.derive_size(48),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 + 50),
                        true,
                    )?;

                    draw_hand(gfx, players, i, Point::new(WIDTH as i32/2, HEIGHT as i32/2 -(SPRITE_HEIGHT as i32)/2))?;
                }
            }
            GamePhase::Playing => (),
        }

        //DRAW THE SCREEN AT THE END
        Ok(gfx.show())
    }
}

fn draw_hand(gfx: &mut SDL2Graphics<'_>, players: &[PlayerState], i: usize, center: Point) -> Result<(), String> {
    Ok(if let Some(tex) = gfx.tex_cache.get("CARD") {            
        let p = center.offset(-5, -(SPRITE_HEIGHT as i32)/2);
        gfx.canvas.copy(
            tex,
            rect_card_spritesheet(players[i].hand.map(|hand| hand.0)),
            Rect::new(p.x, p.y, SPRITE_WIDTH, SPRITE_HEIGHT),
        )?;
            
        let p = p.offset(-(SPRITE_WIDTH as i32) - 10, 0);
        gfx.canvas.copy(
            tex,
            rect_card_spritesheet(players[i].hand.map(|hand| hand.1)),
            Rect::new(p.x, p.y, SPRITE_WIDTH, SPRITE_HEIGHT),
        )?;
    })
}

pub fn rect_card_spritesheet(card: Option<Card>) -> Rect {
    match card {
        Some(card) => {
            let x_offset = {
                if card.value == Value::Ace {
                    0
                } else {
                    card.value as i32 + 1
                }
            };
            let y_offset = card.suit as i32;

            Rect::new(
                (x_offset % 13) * SPRITE_WIDTH as i32,
                y_offset * SPRITE_HEIGHT as i32,
                SPRITE_WIDTH,
                SPRITE_HEIGHT,
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