use super::{GameMessage, GameActionQueue};

#[derive(Debug, Default)]
pub struct EmptyQueue {}

impl GameActionQueue for EmptyQueue {
    fn add(&mut self, _msg: GameMessage) {}
}