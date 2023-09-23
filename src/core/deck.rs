use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::core::card;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<card::Card>,
}

impl Deck {
    ///Returns a new shuffled deck
    pub fn new() -> Self {
        let mut v = Vec::new();

        for suit in card::SUITS {
            for value in card::VALUES {
                v.push(card::Card{ suit, value });
            }
        }

        v.shuffle(&mut thread_rng());

        Deck { cards: v }
    }

    ///Removes and retrieves a card from the deck
    pub fn take(&mut self) -> Option<card::Card> {
        self.cards.pop()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}