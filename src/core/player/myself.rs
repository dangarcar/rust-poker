use std::sync::mpsc;

use crate::core::{error::EngineError, state::GameState};

use super::*;

#[derive(Debug)]
pub struct MyselfPlayer {
    hand: Option<PlayerHand>,
    rx: mpsc::Receiver<PlayerAction>,
}

impl Player for MyselfPlayer {
    fn play(&mut self, _: &GameState, _: usize) -> Result<PlayerAction, EngineError> {
        self.rx.recv().map_err(|_| EngineError::RecvMyselfError)
    }

    fn cards(&self) -> Option<PlayerHand> {
        self.hand
    }

    fn give_cards(&mut self, hand: PlayerHand) {
        self.hand = Some(hand);
    }

    fn blind(&mut self, _: &GameState, _: usize) -> Result<PlayerAction, EngineError> {
        self.rx.recv().map_err(|_| EngineError::RecvMyselfError)
    }
}

impl MyselfPlayer {
    pub fn new(rx: mpsc::Receiver<PlayerAction>) -> Self {
        MyselfPlayer { hand: None, rx }
    }
}
