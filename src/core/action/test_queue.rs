use super::{GameMessage, GameActionQueue};

#[derive(Debug, Default)]
pub struct TestQueue {
    queue: Vec<GameMessage>,
}

impl GameActionQueue for TestQueue {
    fn add(&mut self, msg: GameMessage) {
        println!("{:?}", msg.action);
        self.queue.push(msg);
    }
}