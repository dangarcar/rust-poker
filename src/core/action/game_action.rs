use crate::{player::{PlayerHand, PlayerAction}, core::{state::Round, card::Card, EngineError}};

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
}