use crate::core::{action::game_action::GameAction, hand::Hand, rank::Rankable, player::PlayerHand};

use super::{GameMessage, GameActionQueue};

#[derive(Debug, Default)]
pub struct TestQueue {
    queue: Vec<GameMessage>,
    cards: Vec<PlayerHand>,
}

impl GameActionQueue for TestQueue {
    fn add(&mut self, msg: GameMessage) {
        match msg.action {
            GameAction::DealStartHand { hand, i: _ } => {
                self.cards.push(hand);
            }
            GameAction::PlayedBet { action, i, all_in } => {
                let hand =  self.cards[i];
                let rank = Hand::new_from_hand(hand, &msg.state.community).rank().ok();
                println!("{i} {:?} with hand {:?} and rank {:?}. {:?}", action, hand, rank, all_in);
            },
            GameAction::PlayedFolded { action: _ , i } => {
                let hand =  self.cards[i];
                let rank = Hand::new_from_hand(hand, &msg.state.community).rank().ok();
                println!("Player {i} folded with hand {:?} and rank {:?}", hand, rank);
            }
            _ => println!("{:?}", msg.action)
        }
        self.queue.push(msg);
    }
}

#[derive(Debug, Default)]
pub struct EmptyQueue {}

impl GameActionQueue for EmptyQueue {
    fn add(&mut self, _: GameMessage) {}
}