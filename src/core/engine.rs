use crate::core::card::Card;

use crate::player::Player;

use super::hand::Hand;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Round {
    Starting,
    Preflop,
    Flop,
    Turn,
    River,
    Showdown,
    Complete,
}

impl Round {
    pub fn advance(&self) -> Self {
        match *self {
            Round::Starting => Round::Preflop,
            Round::Preflop => Round::Flop,
            Round::Flop => Round::Turn,
            Round::Turn => Round::River,
            Round::River => Round::Showdown,
            Round::Showdown => Round::Complete,
            Round::Complete => Round::Complete,
        }
    }
}

pub struct GameState {
    round: Round,
    community: Vec<Card>,
    players: Vec<Box<dyn Player>>,
    
    bet_amount: i32,
    pot: i32,
    player_all_in: Option<Box<dyn Player>>,
}