use crate::{player::{PlayerHand, PlayerAction}, core::{state::Round, card::Card, EngineError, rank::Rank}};

#[derive(Debug, Clone)]
pub enum GameAction {
    ///For the initial two cards
    DealStartHand {
        hand: PlayerHand,
        i: usize,
    },

    ///The start of the round 
    RoundChanged {
        round: Round, 
    },

    ///Community card dealt
    DealCommunity {
        card: Card,
    },

    ///Player bet
    PlayedBet {
        action: PlayerAction,
        i: usize,
        all_in: bool,
    },

    ///Player folded
    PlayedFolded {
        action: PlayerAction,
        i: usize,
    },

    ///Player folded
    ErroredPlay {
        error: EngineError,
        i: usize,
    },

    ///Showdown
    ShowdownHand {
        hand: PlayerHand,
        rank: Rank,
        i: usize,
    },

    ///Win game
    WinGame {
        rank: Rank,
        i: usize,
        pot: i32,
    }
}