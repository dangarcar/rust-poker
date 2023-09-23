use crate::core::card::Card;

pub trait Player {
    fn cards(&self) -> (Card, Card);
    fn cash(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum PlayerAction {
    /// Folds the current hand.
    Fold,
    /// Bets the specified amount of money.
    Bet(i32),
}