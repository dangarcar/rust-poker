use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::core::card;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<card::Card>,
}

impl Deck {
    ///Returns a new shuffled deck
    pub fn default() -> Self {
        let mut v = Vec::new();

        for suit in card::SUITS {
            for value in card::VALUES {
                v.push(card::Card{ suit, value });
            }
        }

        v.shuffle(&mut thread_rng());

        Deck { cards: v }
    }

    pub fn new_without_cards(cards: &[card::Card]) -> Self {
        let mut v = Vec::new();

        for suit in card::SUITS {
            for value in card::VALUES {
                let c = card::Card{ suit, value };
                if !cards.contains(&c) {
                    v.push(c);
                }
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

#[cfg(test)]
mod tests {
    use crate::core::card::Card;

    use super::Deck;

    #[test]
    fn cards() {
        let c = vec![
            Card{suit:crate::core::card::Suit::Club, value:crate::core::card::Value::Ace}, 
            Card{suit:crate::core::card::Suit::Diamond, value:crate::core::card::Value::Ace}
        ];
        let deck = Deck::new_without_cards(&c);
        println!("{:?}", deck);
    }
}