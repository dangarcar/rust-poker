use crate::core::player::PlayerHand;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerState {
    pub name: String,
    pub hand: Option<PlayerHand>,
    pub cash: i32,
    pub bet: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerAction {
    Fold,
    Call,
    Raise(i32),
}
