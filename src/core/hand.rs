use crate::core::card::*;
use crate::core::rank;

use super::EngineError;

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Self {
        Hand{cards: Vec::new()}
    }

    pub fn new_from_cards(cards: Vec<Card>) -> Self {
        Hand{cards}
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl rank::Rankable for Hand {
    fn cards(&self) -> Result<Vec<Card>, EngineError> {
        if self.cards.len() < 5 {
            Err(EngineError::SmallHandError)
        }
        else {
            Ok(self.cards.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{card::*, rank::*, hand::Hand};

    #[test]
    fn high_card_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Jack),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::HighCard(Value::Ace))
    }

    #[test]
    fn one_pair_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Jack),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Jack),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::OnePair(Value::Jack))
    }

    #[test]
    fn two_pair_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::TwoPair(Value::Ace, Value::Six))
    }

    #[test]
    fn three_of_a_kind_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Ace),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::ThreeOfAKind(Value::Ace))
    }

    #[test]
    fn four_of_a_kind_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Ace),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::FourOfAKind(Value::Ace))
    }

    #[test]
    fn flush_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Jack),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::Flush(Value::Ace, Suit::Spade))
    }

    #[test]
    fn full_house_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Ten),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::FullHouse(Value::Ten, Value::Ace))
    }

    #[test]
    fn straight_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Queen),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Jack),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::Straight(Value::Ace))
    }

    #[test]
    fn straight_corner_case_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Spade, crate::core::card::Value::Two),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Three),
            Card::new(crate::core::card::Suit::Heart, crate::core::card::Value::Four),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Five),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Jack),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::Straight(Value::Five))
    }

    #[test]
    fn straight_flush_rank() {
        let hand = Hand::new_from_cards(vec![
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Ace),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Queen),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Jack),
            Card::new(crate::core::card::Suit::Diamond, crate::core::card::Value::Ten),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::King),
            Card::new(crate::core::card::Suit::Club, crate::core::card::Value::Six),
        ]);

        let rank = hand.rank().unwrap();
        assert_eq!(rank, Rank::StraightFlush(Value::Ace, Suit::Diamond))
    }
}