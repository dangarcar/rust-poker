use crate::core::{card::Card, state::GameState, error::EngineError};

pub type PlayerHand = (Card, Card);

pub trait Player: std::fmt::Debug {
    fn cards(&self) -> Option<PlayerHand>;
    fn give_cards(&mut self, hand: PlayerHand);

    fn blind(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError>;

    fn play(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum PlayerAction {
    Fold,
    Raise(i32),
    Call(i32),
}

pub mod dummy;
pub mod montecarlo;