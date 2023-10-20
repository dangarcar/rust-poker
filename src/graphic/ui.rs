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
    community_renderer::CommunityRenderer,
    player_render::PlayerRenderer,
    ui_component::{Drawable, EventReceiver},
    SDL2Graphics,
};

#[derive(Default)]
pub struct UI {
    pub player_controller: SelfController,
    pub players: HashMap<usize, PlayerRenderer>,
    pub community: CommunityRenderer,
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

        self.community.draw(gfx)?;

        Ok(())
    }
}

impl UI {
    pub fn start(
        &mut self,
        player_states: &[PlayerState],
        myself: usize,
    ) -> Result<(), EngineError> {
        let mut places = vec![
            Point::new(500, 770),
            Point::new(400, 540),
            Point::new(500, 300),
            Point::new(960, 250),
            Point::new(1420, 300),
            Point::new(1520, 540),
            Point::new(1420, 770),
        ];

        for i in (0..myself).rev() {
            let place = places.remove(0);
            let v = PlayerRenderer::new(place, player_states[i].clone());
            self.players.insert(i, v);
        }
        for i in (myself+1)..player_states.len() {
            let place = places.remove(places.len()-1);
            let v = PlayerRenderer::new(place, player_states[i].clone());
            self.players.insert(i, v);
        }

        self.player_controller.set_state(player_states[myself].clone());

        Ok(())
    }

    pub fn update_states(&mut self, player_states: &[PlayerState], myself: usize) {
        for (i, p) in player_states.iter().enumerate() {
            if i == myself {
                self.player_controller.set_state(p.clone());
            } else {
                self.players.get_mut(&i).unwrap().set_state(p.clone());
            }
        }
    }
}
