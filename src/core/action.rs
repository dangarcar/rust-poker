use std::fmt::Debug;

use game_action::GameAction;

use super::state::GameState;

pub mod game_action;
pub mod test_queue;
pub mod empty_queue;

#[derive(Debug, Clone)]
pub struct GameMessage {
    pub action: GameAction,
    pub state: GameState,
}

impl GameMessage {
    pub fn new(action: GameAction, state: GameState) -> Self {
        GameMessage { action, state }
    }
}

pub trait GameActionQueue: Debug {
    fn add(&mut self, msg: GameMessage);
}