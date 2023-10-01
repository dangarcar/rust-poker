use rand::{thread_rng, Rng};

use crate::core::{state::GameState, EngineError, hand::Hand, rank::Rankable};

use super::*;

#[derive(Debug, Default)]
pub struct DummyPlayer {
    hand: Option<PlayerHand>,
}

const FOLD_PROB: f64 = 0.2;
const RAISE_PROB: f64 = 0.1;

impl Player for DummyPlayer {

    fn play(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError> {
        let mut rng = thread_rng();

        let cash = state.players_money[i];
        let my_bet = state.players_bet[i];
        let diff = state.bet_amount - my_bet; //The amount to call

        let hand = Hand::new_from_hand(self.hand.unwrap(), &state.community);
        let rank = hand.rank()?;
        let fold_prob = FOLD_PROB.powi(rank.to_i32());
        let raise_prob = RAISE_PROB * rank.to_i32() as f64;

        if rng.gen_bool(fold_prob) {
            return Ok(PlayerAction::Fold);
        }
        else if rng.gen_bool(raise_prob) && cash > diff {
            let x: f64 = rng.gen();
            let delta = (cash-diff) as f64 * x*x*x;
            let raised = if delta <= 1.0 {1} else {delta as i32};

            Ok(PlayerAction::Raise(diff + raised))
        }
        else {
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
            return Ok(PlayerAction::Fold);
        }
        else {
            Ok(PlayerAction::Call(state.bet_amount))
        }
    }
}