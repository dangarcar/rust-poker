use itertools::Itertools;
use sdl2::{event::Event, rect::Point};

use crate::game::{player_state::PlayerState, self_controller::SelfController};

use super::{
    player_render::PlayerRenderer,
    ui_component::{Drawable, EventReceiver},
    SDL2Graphics,
};

pub struct UI {
    pub player_controller: SelfController,
    pub players: Vec<PlayerRenderer>,
}

impl UI {
    pub fn new() -> Self {
        let places: &[Point; 7] = &[
            Point::new(960, 250),
            Point::new(400, 540),
            Point::new(1520, 540),
            Point::new(500, 300),
            Point::new(1420, 300),
            Point::new(500, 780),
            Point::new(1420, 780),
        ];

        let players = places.iter()
            .map(|&p| PlayerRenderer::new(p,PlayerState::default()) )
            .collect_vec();

        UI {
            player_controller: SelfController::default(),
            players,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        if let Some(s) = self.player_controller.handle_event(event) {
            println!("{s:?}");
        }
        Ok(())
    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.player_controller.draw(gfx)?;

        for p in &self.players {
            p.draw(gfx)?;
        }

        Ok(())
    }
}
