use crate::core::{player::PlayerHand, rank::Rank};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerState {
    pub name: String,

    pub hand: Option<PlayerHand>,
    pub rank: Option<Rank>,

    pub cash: i32,
    pub bet: i32,

    pub turn: bool,
    pub can_raise: bool,
    pub folded: bool,
    pub all_in: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerAction {
    Fold,
    Call,
    Raise(i32),
}
