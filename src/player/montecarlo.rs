use std::{cmp, ops::Div};

use rayon::prelude::*;
use rand::{thread_rng, Rng};
use crate::core::{state::GameState, EngineError, engine::Engine, action::empty_queue::EmptyQueue};
use super::{*, dummy::DummyPlayer};

type Rival = DummyPlayer;
const SIM_ROUNDS: i32 = 10000;

#[derive(Debug, Default)]
pub struct MontecarloPlayer {
    hand: Option<PlayerHand>,
}

impl Player for MontecarloPlayer {
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

    fn play(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError> {
        let mut rng = thread_rng();

        let cash = state.players_money[i];
        let my_bet = state.players_bet[i];
        let diff = state.bet_amount - my_bet; //The amount to call

        let (win, _lose) = self.montecarlo_sim(state, i, SIM_ROUNDS)?;

        let win_pp = win * state.num_active_players as f64;

        if win_pp < 1.0 {
            return Ok(PlayerAction::Fold);
        }
        else if rng.gen_bool((win_pp-1.0).div(state.num_active_players as f64).min(1.0)) && cash > diff {
            let x = (rng.gen::<f64>() + win*2.0) / 4.0;
            let delta = (cash-diff) as f64 * x*x*x;
            let raised = if delta <= 1.0 {1} else {delta as i32};

            Ok(PlayerAction::Raise(diff + raised))
        }
        else {
            Ok(PlayerAction::Call(diff))
        }
    }
}

impl MontecarloPlayer {
    fn play_montecarlo_game(&self, state: &GameState, i: usize) -> Result<i32, EngineError> {
        let initial_cash = state.players_money[i];

        let mut players: Vec<Box<dyn Player>> = Vec::new();
        let players_length = state.players_money.len();
        for _ in 0..players_length {
            players.push(Box::new(Rival::default()));
        }

        let res = Engine::run_from_game_state(players, Box::new(EmptyQueue::default()), state.clone(), self.hand.unwrap(), i);
        let end_cash = res?[i];

        Ok(end_cash - initial_cash)
    }

    pub fn montecarlo_sim(&self, state: &GameState, i: usize, rounds: i32) -> Result<(f64,f64), EngineError> {
        let times = 0..rounds;
        let win: i32 = times
            .into_par_iter()
            .map(|_| {
                let diff = self.play_montecarlo_game(state, i).unwrap_or(-1);
                if diff > 0 { 1 }
                else { 0 }
            })
            .sum();

        let win_prob = win as f64 / rounds as f64;

        Ok((win_prob, 1.0-win_prob))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{state::*, card::*, action::test_queue::TestQueue};

    use super::*;

    use std::{sync::Once, time::Instant};

    static INIT: Once = Once::new();

    #[test]
    fn montecarlo_game() -> Result<(), EngineError>{
        INIT.call_once(env_logger::init);

        let mut players: Vec<Box<dyn Player>> = Vec::new();
        let n = 8;

        for _ in 0..n {
            players.push(Box::new(MontecarloPlayer::default()));
        }
        let old_stacks = vec![1000; n];

        let engine = Engine::new(players, Box::new(TestQueue::default()))?;

        let new_stacks =  engine.run(old_stacks.clone(), 1)?;

        println!("{:?}", old_stacks);
        println!("{:?}", new_stacks);

        Ok(())
    }

    #[test]
    fn montecarlo_simulation() {
        INIT.call_once(env_logger::init);
            let state = crate::core::state::GameState { 
                round: Round::Flop, 
                community: vec![Card{ suit: Suit::Diamond, value: Value::Ten }, Card{ suit: Suit::Spade, value: Value::Two }, Card{ suit: Suit::Heart, value: Value::King },], 
                players_bet: vec![1, 1, 1, 1], 
                players_money: vec![0, 99, 99, 99], 
                bet_amount: 1, 
                players_all_in: vec![0], 
                folded_players: vec![],
                num_active_players: 3, 
                active_players: vec![1, 2, 3],
            };
            let player_idx = 1;
    
            let m = MontecarloPlayer { hand: Some((Card{ suit: Suit::Club, value: Value::Two }, Card{ suit: Suit::Diamond, value: Value::Ace})) };
    
            for i in 0..6 {
                let r = 10i32.pow(i);
                let t = Instant::now();
                let p = m.montecarlo_sim(&state, player_idx, r).unwrap();
                let d = t.elapsed();
                println!("{r} rounds: {d:?} => {p:?}");
            }
    }

    #[test]
    fn loop_montecarlo() {
        INIT.call_once(env_logger::init);
        let rounds = 2000;

        let mut v = 0;

        for _ in 0..rounds {
            let state = crate::core::state::GameState { 
                round: Round::Flop, 
                community: vec![Card{ suit: Suit::Diamond, value: Value::Ten }, Card{ suit: Suit::Spade, value: Value::Two }, Card{ suit: Suit::Heart, value: Value::King },], 
                players_bet: vec![1, 1, 1, 1], 
                players_money: vec![0, 99, 99, 99], 
                bet_amount: 1, 
                players_all_in: vec![0], 
                folded_players: vec![],
                num_active_players: 3, 
                active_players: vec![1, 2, 3],
            };
            let player_idx = 1;
    
            let m = MontecarloPlayer { hand: Some((Card{ suit: Suit::Club, value: Value::Two }, Card{ suit: Suit::Diamond, value: Value::Ace})) };
    
            let diff = m.play_montecarlo_game(&state, player_idx).unwrap();
    
            if diff > 0 { v += 1; }
        }

        println!("{}", v as f64 / rounds as f64 * 100.0);
    }
}