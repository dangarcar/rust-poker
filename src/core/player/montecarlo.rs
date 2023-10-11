use std::ops::Div;

use crate::core::{card::*, deck::*, error::EngineError, hand::*, rank::*, state::GameState};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

use super::*;

const SIM_ROUNDS: i32 = 5000;

const BLIND_FOLD_PROB: f64 = 0.7;

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
        let n_players = state.players_money.len() - state.folded_players.len();

        let (win, _lose) = self.montecarlo_sim(state, i, SIM_ROUNDS)?;
        let fold = (win * n_players as f64) < BLIND_FOLD_PROB;

        if fold || cash < state.bet_amount {
            return Ok(PlayerAction::Fold);
        } else {
            Ok(PlayerAction::Call(state.bet_amount))
        }
    }

    fn play(&mut self, state: &GameState, i: usize) -> Result<PlayerAction, EngineError> {
        let n_players = state.players_money.len() - state.folded_players.len();
        let mut rng = thread_rng();

        let cash = state.players_money[i];
        let my_bet = state.players_bet[i];
        let diff = state.bet_amount - my_bet; //The amount to call

        let (win, _lose) = self.montecarlo_sim(state, i, SIM_ROUNDS)?;

        let win_pp = win * n_players as f64;

        if win_pp < 1.0 {
            return Ok(PlayerAction::Fold);
        } else if rng.gen_bool((win_pp - 1.0).div(state.num_active_players as f64).min(1.0))
            && cash > diff
        {
            let x = (rng.gen::<f64>() + win * 2.0) / 4.0;
            let delta = (cash - diff) as f64 * x * x * x;
            let raised = if delta <= 1.0 { 1 } else { delta as i32 };

            Ok(PlayerAction::Raise(diff + raised))
        } else {
            Ok(PlayerAction::Call(diff))
        }
    }
}

impl MontecarloPlayer {
    fn play_montecarlo(&self, state: &GameState, player_idx: usize) -> Result<usize, EngineError> {
        let players_length = state.players_money.len();

        let mut players_hands = Vec::new();

        let mut community = state.community.clone();

        let h = self.hand.unwrap();
        let mut deck = Deck::new_without_cards(&[&[h.0, h.1], community.as_slice()].concat());

        //Give cards to the players
        for i in 0..players_length {
            if i == player_idx {
                players_hands.push(self.hand.unwrap());
            } else {
                players_hands.push((
                    deck.take().ok_or(EngineError::BadDeckError)?,
                    deck.take().ok_or(EngineError::BadDeckError)?,
                ));
            }
        }

        //Fill the community cards
        while community.len() < 5 {
            community.push(deck.take().ok_or(EngineError::BadDeckError)?);
        }

        let mut pos = (0..players_length).collect_vec();
        pos.retain(|p| !state.folded_players.contains(p));

        let mut winner = (Rank::HighCard(Value::Two), usize::MAX); //The worst possible rank
        for i in pos {
            let hand = Hand::new_from_hand(players_hands[i], &community);

            let rank = hand.rank()?;
            if winner.0 < rank {
                winner.1 = i;
                winner.0 = rank;
            }
        }

        Ok(winner.1)
    }

    pub fn montecarlo_sim(
        &self,
        state: &GameState,
        i: usize,
        rounds: i32,
    ) -> Result<(f64, f64), EngineError> {
        let times = 0..rounds;
        let win: i32 = times
            .into_par_iter()
            .map(|_| {
                let p = self.play_montecarlo(state, i).unwrap_or(usize::MAX);
                if p == i {
                    1
                } else {
                    0
                }
            })
            .sum();

        let win_prob = win as f64 / rounds as f64;

        Ok((win_prob, 1.0 - win_prob))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{action::test_queue::TestQueue, engine::Engine, state::*};

    use super::*;

    use std::time::Instant;

    #[test]
    fn montecarlo_game() -> Result<(), EngineError> {
        let mut players: Vec<Box<dyn Player>> = Vec::new();
        let n = 8;

        for _ in 0..n {
            players.push(Box::new(MontecarloPlayer::default()));
        }
        let old_stacks = vec![1000; n];

        let engine = Engine::new(players, Box::new(TestQueue::default()))?;

        let new_stacks = engine.run(old_stacks.clone(), 1)?;

        println!("{:?}", old_stacks);
        println!("{:?}", new_stacks);

        Ok(())
    }

    #[test]
    fn montecarlo_simulation() {
        let state = crate::core::state::GameState {
            round: Round::Flop,
            community: vec![
                Card {
                    suit: Suit::Diamond,
                    value: Value::Ten,
                },
                Card {
                    suit: Suit::Spade,
                    value: Value::Two,
                },
                Card {
                    suit: Suit::Heart,
                    value: Value::King,
                },
            ],
            players_bet: vec![1, 1, 1, 1],
            players_money: vec![0, 99, 99, 99],
            bet_amount: 1,
            players_all_in: vec![0],
            folded_players: vec![],
            num_active_players: 3,
            active_players: vec![1, 2, 3],
        };
        let player_idx = 1;

        let m = MontecarloPlayer {
            hand: Some((
                Card {
                    suit: Suit::Club,
                    value: Value::Two,
                },
                Card {
                    suit: Suit::Diamond,
                    value: Value::Ace,
                },
            )),
        };

        for i in 0..7 {
            let r = 10i32.pow(i);
            let t = Instant::now();
            let p = m.montecarlo_sim(&state, player_idx, r).unwrap();
            let d = t.elapsed();
            println!("{r} rounds: {d:?} => {p:?}");
        }
    }

    #[test]
    fn loop_montecarlo() {
        let rounds = 2000;

        let mut v = 0;

        for _ in 0..rounds {
            let state = crate::core::state::GameState {
                round: Round::Flop,
                community: vec![
                    Card {
                        suit: Suit::Diamond,
                        value: Value::Ten,
                    },
                    Card {
                        suit: Suit::Spade,
                        value: Value::Two,
                    },
                    Card {
                        suit: Suit::Heart,
                        value: Value::King,
                    },
                ],
                players_bet: vec![1, 1, 1, 1],
                players_money: vec![0, 99, 99, 99],
                bet_amount: 1,
                players_all_in: vec![0],
                folded_players: vec![],
                num_active_players: 3,
                active_players: vec![1, 2, 3],
            };
            let player_idx = 1;

            let m = MontecarloPlayer {
                hand: Some((
                    Card {
                        suit: Suit::Club,
                        value: Value::Two,
                    },
                    Card {
                        suit: Suit::Diamond,
                        value: Value::Ace,
                    },
                )),
            };

            let p = m.play_montecarlo(&state, player_idx).unwrap();

            if p == player_idx {
                v += 1;
            }
        }

        println!("{}", v as f64 / rounds as f64 * 100.0);
    }
}
