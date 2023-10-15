use std::sync::mpsc;

use super::{GameActionQueue, GameMessage};

#[derive(Debug)]
pub struct MpscQueue {
    tx: mpsc::Sender<GameMessage>,
}

impl GameActionQueue for MpscQueue {
    fn add(&mut self, msg: GameMessage) {
        self.tx.send(msg).ok();
    }
}

impl MpscQueue {
    pub fn new(tx: mpsc::Sender<GameMessage>) -> Self {
        MpscQueue { tx }
    }
}
