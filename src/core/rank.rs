use std::cmp::Reverse;

use itertools::Itertools;

use crate::core::card::*;

use super::error::EngineError;

#[derive(PartialOrd, Ord, Eq, Debug, Clone, Copy, Hash)]
pub enum Rank {
    ///The highest card in the hand
    HighCard(Value),
    ///Two cards with same value but different suits
    OnePair(Value),
    ///Two different pairs, the bigger value pair the first
    TwoPair(Value, Value),
    ///Three cards with same value but different suits
    ThreeOfAKind(Value),
    ///Five cards that follows each other.
    ///<br>The enum contains the highest value of the straight
    Straight(Value),
    ///Five cards with the same suit
    ///<br>The enum contains the highest value of the flush and the suit
    Flush(Value, Suit),
    ///A three of a kind and a pair
    ///<br>The enum contains the value of the three and the pair
    FullHouse(Value, Value),
    ///Four cards with same value but different suits
    FourOfAKind(Value),
    ///A straight and a flush
    ///<br>The enum contains the highest value of the straght flush and the suit
    StraightFlush(Value, Suit),
}

impl Rank {
    pub fn to_i32(&self) -> i32 {
        match self {
            Rank::HighCard(_) => 1,
            Rank::OnePair(_) => 2,
            Rank::TwoPair(_,_) => 3,
            Rank::ThreeOfAKind(_) => 4,
            Rank::Straight(_) => 5,
            Rank::Flush(_,_) => 6,
            Rank::FullHouse(_,_) => 7,
            Rank::FourOfAKind(_) => 8,
            Rank::StraightFlush(_,_) => 9,
        }
    }
}

impl PartialEq for Rank {
    ///Custom eq to not order dependent on the suit
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::HighCard(l0), Self::HighCard(r0)) => l0 == r0,
            (Self::OnePair(l0), Self::OnePair(r0)) => l0 == r0,
            (Self::TwoPair(l0, l1), Self::TwoPair(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::ThreeOfAKind(l0), Self::ThreeOfAKind(r0)) => l0 == r0,
            (Self::Straight(l0), Self::Straight(r0)) => l0 == r0,
            (Self::Flush(l0, _), Self::Flush(r0, _)) => l0 == r0,
            (Self::FullHouse(l0, l1), Self::FullHouse(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::FourOfAKind(l0), Self::FourOfAKind(r0)) => l0 == r0,
            (Self::StraightFlush(l0, _), Self::StraightFlush(r0, _)) => l0 == r0,
            _ => false,
        }
    }
}

pub trait Rankable {
    ///A copy of the cards of error if vector is empty
    fn cards(&self) -> Result<Vec<Card>, EngineError>;

    fn rank(&self) -> Result<Rank, EngineError> {
        let mut cards = self.cards()?;

        if let Some(a) = rank_straight_flush(&mut cards){ //Straight flush returns also a flush if there isn't any straights
            Ok(a)
        }
        else if let Some(a) = rank_four_of_a_kind(&mut cards){
            Ok(a)
        }
        else if let Some(a) = rank_full_house(&mut cards){
            Ok(a)
        }
        else if let Some(a) = rank_straight(&mut cards){
            Ok(a)
        }
        else if let Some(a) = rank_three_of_a_kind(&mut cards){
            Ok(a)
        }
        else if let Some(a) = rank_two_pair(&mut cards){
            Ok(a)
        }
        else if let Some(a) = rank_one_pair(&mut cards){
            Ok(a)
        }
        else{
            cards.sort_by_key(|c| c.value);
            let highest = cards.pop().ok_or(EngineError::HighestCardNotAvailable)?;
            Ok(Rank::HighCard(highest.value))
        }
    }
}

///Returns Rank::StraightFlush if it matches, Rank::Flush if there isn't straight or None otherwise
fn rank_straight_flush(cards: &mut[Card]) -> Option<Rank> {
    let flush = rank_flush(cards);

    //Check for flush
    if let Some(Rank::Flush(flush, suit)) = flush { 
        let i = cards.iter().position(|&c| c.value == flush && c.suit == suit).unwrap();
        let mut j = cards.len();
        for k in i..cards.len() {
            if cards[i].suit != suit {
                j = k;
                break;
            }
        }

        //Check for straight within the flush cards
        if let Some(Rank::Straight(straight)) = rank_straight(& mut cards[i..j]) {
            return Some(Rank::StraightFlush(straight, suit))
        }
        else {
            return Some(Rank::Flush(flush, suit))
        }
    }

    None
}

///Returns Rank::FourOfAKind it matches or None otherwise
fn rank_four_of_a_kind(cards: &mut[Card]) -> Option<Rank> {
    cards.sort_by_key(|c| Reverse(c.value));

    for i in 0..cards.len()-3 {
        if cards[i].value == cards[i+1].value 
        && cards[i].value == cards[i+2].value 
        && cards[i].value == cards[i+3].value {
            return Some(Rank::FourOfAKind(cards[i].value))
        }
    }

    None
}

///Returns Rank::FullHouse it matches or None otherwise
fn rank_full_house(cards: &mut[Card]) -> Option<Rank> {
    let o = rank_three_of_a_kind(cards);
    
    if let Some(Rank::ThreeOfAKind(three)) = o {
        for i in 0..cards.len()-1 {
            if cards[i].value == cards[i+1].value 
            && cards[i].value != three {
                return Some(Rank::FullHouse(three, cards[i].value))
            }
        }
    }

    None
}

///Returns Rank::Flush it matches or None otherwise
fn rank_flush(cards: &mut[Card]) -> Option<Rank> {
    cards.sort_by_key(|c| c.suit);

    for i in 0..cards.len()-4 {
        if cards[i].suit == cards[i+1].suit 
        && cards[i].suit == cards[i+2].suit 
        && cards[i].suit == cards[i+3].suit
        && cards[i].suit == cards[i+4].suit {
            return Some(Rank::Flush(cards[i].value, cards[i].suit))
        }
    }

    None
}

///Returns Rank::Straight it matches or None otherwise
fn rank_straight(cards: &mut[Card]) -> Option<Rank> {
    cards.sort_by_key(|c| Reverse(c.value));
    let cards = &cards.iter().dedup_by(|a, b| a.value == b.value).collect_vec()[..];
    
    if cards.len() < 5 {
        return None
    }

    for i in 0..cards.len()-4 {
        if cards[i].value == Value::Ace {
            let l = cards.len();
            if cards[l-1].value == Value::Two 
            && cards[l-2].value == Value::Three 
            && cards[l-3].value == Value::Four 
            && cards[l-4].value == Value::Five {
                return Some(Rank::Straight(Value::Five))
            }
        }
        
        let mut straight = true;
        let v = cards[i].value as i32;
        for j in 1..5 {
            if v != cards[i+j].value as i32 + j as i32 {
                straight = false;
                break;
            }
        }

        if straight == true {
            return Some(Rank::Straight(cards[i].value))
        }
    }

    None
}

///Returns Rank::ThreeOfAKind it matches or None otherwise
fn rank_three_of_a_kind(cards: &mut[Card]) -> Option<Rank> {
    cards.sort_by_key(|c| Reverse(c.value));

    for i in 0..cards.len()-2 {
        if cards[i].value == cards[i+1].value && cards[i].value == cards[i+2].value {
            return Some(Rank::ThreeOfAKind(cards[i].value))
        }
    }

    None
}

///Returns Rank::TwoPair it matches or None otherwise
fn rank_two_pair(cards: &mut[Card]) -> Option<Rank> {
    cards.sort_by_key(|c| Reverse(c.value));

    let mut first = None;
    for i in 0..cards.len()-1 {
        if cards[i].value == cards[i+1].value {
            if let Some(v) = first {
                return Some(Rank::TwoPair(v, cards[i].value))
            }
            else {
                first = Some(cards[i].value);
            }
        }
    }

    None
}

///Returns Rank::OnePair it matches or None otherwise
fn rank_one_pair(cards: &mut[Card]) -> Option<Rank> {
    cards.sort_by_key(|c| c.value);

    for i in 0..cards.len()-1 {
        if cards[i].value == cards[i+1].value {
            return Some(Rank::OnePair(cards[i].value))
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::core::rank::Rank;
    use crate::core::card::Value;

    #[test]
    fn correct_rank_order(){
        assert!(Rank::HighCard(Value::Ace) < Rank::OnePair(Value::Ace));
        assert!(Rank::OnePair(Value::Ace) < Rank::TwoPair(Value::Ace, Value::Eight));
        assert!(Rank::TwoPair(Value::Ace, Value::Eight) < Rank::ThreeOfAKind(Value::Ace));
        assert!(Rank::ThreeOfAKind(Value::Ace) < Rank::Straight(Value::Ace));
        assert!(Rank::Straight(Value::Ace) < Rank::Flush(Value::Ace, crate::core::card::Suit::Club));
        assert!(Rank::Flush(Value::Ace, crate::core::card::Suit::Club) < Rank::FullHouse(Value::Ace, Value::Eight));
        assert!(Rank::FullHouse(Value::Ace, Value::Eight) < Rank::FourOfAKind(Value::Ace));
        assert!(Rank::FourOfAKind(Value::Ace) < Rank::StraightFlush(Value::Ace, crate::core::card::Suit::Diamond));
    }

    #[test]
    fn two_pair_cmp() {
        assert!(Rank::TwoPair(Value::Ace, Value::Eight) > Rank::TwoPair(Value::Ace, Value::Six));
        assert!(Rank::TwoPair(Value::Ten, Value::Eight) < Rank::TwoPair(Value::Ace, Value::Six));
    }

    #[test]
    fn flush_eq() {
        assert_eq!(Rank::Flush(Value::Ace, crate::core::card::Suit::Club), Rank::Flush(Value::Ace, crate::core::card::Suit::Diamond));
    }
}