use log::debug;
use rand::{thread_rng, Rng};

use crate::core::{state::GameState, EngineError};

use super::*;

#[derive(Debug, Default)]
pub struct DummyPlayer {
    hand: Option<PlayerHand>,
}

const FOLD_PROB: f64 = 0.2;
const RAISE_PROB: f64 = 0.4;

const MAX_BET: i32 = 100; 

impl Player for DummyPlayer {

    fn play(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError> {
        let mut rng = thread_rng();

        let cash = state.players_money[i];
        let my_bet = state.players_bet[i];
        let diff = state.bet_amount - my_bet; //The amount to raise

        if rng.gen_bool(FOLD_PROB) {
            debug!("Player {i} folded with cards {:?}", self.hand);
            return Ok(PlayerAction::Fold);
        }
        else if rng.gen_bool(RAISE_PROB) && cash > diff {
            let x: f64 = rng.gen();
            let delta = std::cmp::min(cash-diff, MAX_BET) as f64 * x*x*x;
            let raised = if delta <= 1.0 {1} else {delta as i32};

            debug!("Player {i} raised {raised} with cards {:?}", self.hand);
            Ok(PlayerAction::Raise(diff + raised))
        }
        else {
            debug!("Player {i} called {} with cards {:?}", diff, self.hand);
            Ok(PlayerAction::Call(diff))
        }
    }

    fn cards(&self) -> Option<PlayerHand> {
        self.hand
    }

    fn give_cards(&mut self, hand: PlayerHand) {
        self.hand = Some(hand);
    }

    fn blind(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError> {
        let cash = state.players_money[i];

        let fold = (self.hand.unwrap().0.value as i32 + self.hand.unwrap().1.value as i32) < 10;

        if fold || cash < state.bet_amount {
            debug!("Player {i} folded with cards {:?}", self.hand);
            return Ok(PlayerAction::Fold);
        }
        else {
            debug!("Player {i} called {} with cards {:?}", state.bet_amount, self.hand);
            Ok(PlayerAction::Call(state.bet_amount))
        }
    }
}