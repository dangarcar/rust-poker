use std::collections::HashMap;

use sdl2::{event::Event, rect::Point};

use crate::{
    core::error::EngineError,
    game::{
        player_state::{PlayerAction, PlayerState},
        self_controller::SelfController,
    },
};

use super::{
    player_render::PlayerRenderer,
    ui_component::{Drawable, EventReceiver},
    SDL2Graphics,
};

pub struct UI {
    pub player_controller: SelfController,
    pub players: HashMap<usize, PlayerRenderer>,
}

impl EventReceiver<Result<Option<PlayerAction>, String>> for UI {
    fn handle_event(&mut self, event: &Event) -> Result<Option<PlayerAction>, String> {
        Ok(self.player_controller.handle_event(event))
    }
}

impl Drawable for UI {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.player_controller.draw(gfx)?;

        for p in self.players.values() {
            p.draw(gfx)?;
        }

        Ok(())
    }
}

impl UI {
    pub fn new() -> Self {
        UI {
            player_controller: SelfController::default(),
            players: HashMap::new(),
        }
    }

    pub fn start(
        &mut self,
        player_states: &Vec<PlayerState>,
        myself: usize,
    ) -> Result<(), EngineError> {
        let mut places = vec![
            Point::new(960, 250),
            Point::new(400, 540),
            Point::new(1520, 540),
            Point::new(500, 300),
            Point::new(1420, 300),
            Point::new(500, 780),
            Point::new(1420, 780),
        ];

        for (i, p) in player_states.iter().enumerate() {
            if i == myself {
                self.player_controller.set_state(p.clone());
            } else {
                let v =
                    PlayerRenderer::new(places.pop().ok_or(EngineError::BadGameError)?, p.clone());
                self.players.insert(i, v);
            }
        }

        Ok(())
    }
}
