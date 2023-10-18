use log::warn;

use crate::core::card::*;
use crate::core::deck::*;
use crate::core::hand::Hand;
use crate::core::player::*;
use crate::core::rank::Rank;
use crate::core::rank::Rankable;
use crate::core::state::*;

use super::action::game_action::GameAction;
use super::action::GameActionQueue;
use super::action::GameMessage;
use super::error::EngineError;

#[derive(Debug)]
pub struct Engine {
    action_queue: Box<dyn GameActionQueue>,
    deck: Deck,
    pub state: GameState,
    pub players_hands: Vec<(Card, Card)>,
    pub players: Vec<Box<dyn Player>>,
}

impl Engine {
    pub fn new(
        players: Vec<Box<dyn Player>>,
        action_queue: Box<dyn GameActionQueue>,
    ) -> Result<Self, EngineError> {
        let deck = Deck::default();
        let players_hands = Vec::new();

        let state = GameState {
            round: Round::Starting,
            community: Vec::new(),
            players_bet: Vec::new(),
            players_money: Vec::new(),
            bet_amount: 0,
            players_all_in: Vec::new(),
            num_active_players: players.len() as i32,
            active_players: (0..players.len()).collect(),
            folded_players: Vec::new(),
        };

        Ok(Engine {
            action_queue,
            deck,
            state,
            players_hands,
            players,
        })
    }

    pub fn run(mut self, players_money: Vec<i32>, blind: i32) -> Result<Vec<i32>, EngineError> {
        if self.players.len() != players_money.len() {
            return Err(EngineError::BadGameError);
        } else {
            self.state.players_money = players_money;
        }

        if !blind > 0 {
            return Err(EngineError::SmallBlindError);
        } else {
            self.state.bet_amount = blind;
        }

        loop {
            match self.state.round {
                Round::Starting => self.start()?,
                Round::Preflop => self.preflop(0)?,
                Round::Flop => self.flop()?,
                Round::Turn => self.turn()?,
                Round::River => self.river()?,
                Round::Showdown => self.showdown()?,

                Round::Complete => return Ok(self.state.players_money),
            };
        }
    }

    fn start(&mut self) -> Result<(), EngineError> {
        for (i, p) in self.players.iter_mut().enumerate() {
            self.state.players_bet.push(0);

            let hand = (
                self.deck.take().ok_or(EngineError::BadDeckError)?,
                self.deck.take().ok_or(EngineError::BadDeckError)?,
            );
            self.players_hands.push(hand);
            p.give_cards(hand);

            self.action_queue.add(GameMessage::new(
                GameAction::DealStartHand { hand, i },
                self.state.clone(),
            ));
        }

        self.state.round = self.state.round.next();
        Ok(())
    }

    fn preflop(&mut self, start: usize) -> Result<(), EngineError> {
        self.add_action(GameAction::RoundChanged {
            round: self.state.round,
        });

        for i in start..self.players.len() {
            //If there's only one player, there's no need to play
            if self.state.num_active_players <= 1 {
                break;
            }

            let action = self.players[i].blind(&self.state, i);

            match action {
                Ok(PlayerAction::Call(amount)) => {
                    let all_in = self.state.bet_amount
                        >= self.state.players_money[i] + self.state.players_bet[i];
                    self.state.bet(amount, i, all_in)?;

                    self.add_action(GameAction::PlayedBet {
                        action: action.unwrap(),
                        i,
                        all_in,
                    });
                }
                Ok(PlayerAction::Fold) => {
                    self.state.folded_players.push(i);
                    self.state.remove_inactive_players();

                    self.add_action(GameAction::PlayedFolded {
                        action: action.unwrap(),
                        i,
                    });
                }
                Ok(PlayerAction::Raise(_)) => {
                    self.state.folded_players.push(i);
                    self.state.remove_inactive_players();

                    let error = EngineError::NoRaiseAllowedError;
                    self.add_action(GameAction::ErroredPlay { error, i });
                    return Err(error);
                }
                Err(e) => {
                    warn!("{e}");

                    self.state.folded_players.push(i);
                    self.state.remove_inactive_players();

                    self.add_action(GameAction::ErroredPlay { error: e, i });
                }
            }
        }

        self.state.round = self.state.round.next();
        Ok(())
    }

    fn flop(&mut self) -> Result<(), EngineError> {
        self.add_action(GameAction::RoundChanged {
            round: self.state.round,
        });

        self.deal_community(3)?;
        self.betting_round()?;

        self.state.round = self.state.round.next();
        Ok(())
    }

    fn turn(&mut self) -> Result<(), EngineError> {
        self.add_action(GameAction::RoundChanged {
            round: self.state.round,
        });

        self.deal_community(1)?;
        self.betting_round()?;

        self.state.round = self.state.round.next();
        Ok(())
    }

    fn river(&mut self) -> Result<(), EngineError> {
        self.add_action(GameAction::RoundChanged {
            round: self.state.round,
        });

        self.deal_community(1)?;
        self.betting_round()?;

        self.state.round = self.state.round.next();
        Ok(())
    }

    fn showdown(&mut self) -> Result<(), EngineError> {
        self.add_action(GameAction::RoundChanged {
            round: self.state.round,
        });

        let mut winner = (Rank::HighCard(Value::Two), usize::MAX); //The worst possible rank
        let mut pos = self.state.active_players.clone();
        pos.append(&mut self.state.players_all_in);

        for i in pos {
            let hand = Hand::new_from_hand(self.players_hands[i], &self.state.community);

            let rank = hand.rank()?;
            if winner.0 < rank {
                winner.1 = i;
                winner.0 = rank;
            }

            self.add_action(GameAction::ShowdownHand {
                hand: self.players_hands[i],
                rank,
                i,
            });
        }

        let pot = self.state.award(winner.1);
        self.add_action(GameAction::WinGame {
            rank: winner.0,
            i: winner.1,
            pot,
        });

        self.state.round = self.state.round.next();
        Ok(())
    }

    fn betting_round(&mut self) -> Result<(), EngineError> {
        let mut raising = true;

        while raising && self.state.num_active_players > 1 {
            raising = false;

            for i in self.state.active_players.clone() {
                //If there's only one player, there's no need to play
                if self.state.num_active_players <= 1 {
                    self.state.remove_inactive_players();
                    return Ok(());
                }

                let action = self.players[i].play(&self.state, i);

                match action {
                    Ok(PlayerAction::Raise(amount)) => {
                        let all_in = self.state.bet_amount
                            == self.state.players_money[i] + self.state.players_bet[i];
                        self.state.bet(amount, i, all_in)?;

                        raising = true;

                        self.add_action(GameAction::PlayedBet {
                            action: action.unwrap(),
                            i,
                            all_in,
                        });
                    }
                    Ok(PlayerAction::Call(amount)) => {
                        let all_in = self.state.bet_amount
                            >= self.state.players_money[i] + self.state.players_bet[i];
                        self.state.bet(amount, i, all_in)?;

                        self.add_action(GameAction::PlayedBet {
                            action: action.unwrap(),
                            i,
                            all_in,
                        });
                    }
                    Ok(PlayerAction::Fold) => {
                        self.state.folded_players.push(i);
                        self.state.remove_inactive_players();

                        self.add_action(GameAction::PlayedFolded {
                            action: action.unwrap(),
                            i,
                        });
                    }
                    Err(e) => {
                        warn!("{e}");

                        self.state.folded_players.push(i);
                        self.state.remove_inactive_players();

                        self.add_action(GameAction::ErroredPlay { error: e, i });
                    }
                }
            }
        }

        Ok(())
    }

    fn deal_community(&mut self, n: i32) -> Result<(), EngineError> {
        for _ in 0..n {
            let c = self.deck.take().ok_or(EngineError::BadDeckError)?;
            self.state.community.push(c);

            self.add_action(GameAction::DealCommunity { card: c });
        }

        Ok(())
    }

    #[inline(always)]
    fn add_action(&mut self, action: GameAction) {
        self.action_queue
            .add(GameMessage::new(action, self.state.clone()));
    }

    pub fn run_from_game_state(
        players: Vec<Box<dyn Player>>,
        action_queue: Box<dyn GameActionQueue>,
        state: GameState,
        hand: PlayerHand,
        player_idx: usize,
    ) -> Result<Vec<i32>, EngineError> {
        let deck =
            Deck::new_without_cards(&[&[hand.0, hand.1], state.community.as_slice()].concat());
        let players_hands = Vec::new();

        let mut engine = Engine {
            action_queue,
            deck,
            state,
            players_hands,
            players,
        };

        for i in 0..engine.players.len() {
            if i == player_idx {
                engine.players_hands.push(hand);
                engine.players[i].give_cards(hand);
                engine.add_action(GameAction::DealStartHand { hand, i });
            } else {
                let h = (
                    engine.deck.take().ok_or(EngineError::BadDeckError)?,
                    engine.deck.take().ok_or(EngineError::BadDeckError)?,
                );
                engine.players_hands.push(h);
                engine.players[i].give_cards(h);
                engine.add_action(GameAction::DealStartHand { hand, i });
            }
        }

        match engine.state.round {
            Round::Flop | Round::Turn | Round::River => {
                engine.betting_round()?;
                engine.state.round = engine.state.round.next();
            }
            _ => (),
        }

        loop {
            match engine.state.round {
                Round::Starting => engine.start()?,
                Round::Preflop => engine.preflop(player_idx)?,
                Round::Flop => engine.flop()?,
                Round::Turn => engine.turn()?,
                Round::River => engine.river()?,
                Round::Showdown => engine.showdown()?,

                Round::Complete => return Ok(engine.state.players_money),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::action::test_queue::TestQueue,
        core::{action::test_queue::EmptyQueue, player::*},
    };

    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    #[test]
    #[should_panic]
    fn so_many_players() {
        INIT.call_once(env_logger::init);

        let mut players: Vec<Box<dyn Player>> = Vec::new();
        let n = 24;

        for _ in 0..n {
            players.push(Box::new(dummy::DummyPlayer::default()));
        }
        let old_stacks = vec![100; n];

        let engine = Engine::new(players, Box::new(TestQueue::default())).unwrap();

        let new_stacks = engine.run(old_stacks.clone(), 1).unwrap();

        println!("{:?}", old_stacks);
        println!("{:?}", new_stacks);
    }

    #[test]
    fn debug_engine() -> Result<(), EngineError> {
        INIT.call_once(env_logger::init);

        let mut players: Vec<Box<dyn Player>> = Vec::new();
        let n = 23;

        for _ in 0..n {
            players.push(Box::new(dummy::DummyPlayer::default()));
        }
        let old_stacks = vec![1000; n];

        let engine = Engine::new(players, Box::new(TestQueue::default()))?;

        let new_stacks = engine.run(old_stacks.clone(), 1)?;

        println!("{:?}", old_stacks);
        println!("{:?}", new_stacks);

        Ok(())
    }

    #[test]
    #[ignore = "Takes ages for running"]
    fn loop_dummies() -> Result<(), EngineError> {
        INIT.call_once(env_logger::init);

        let rounds = 50;
        let mut wins = vec![0; 4];

        for _ in 0..rounds {
            let initial_money = 100;

            let mut stacks = vec![initial_money; 4];
            let old_stacks = stacks.clone();

            for _ in 0..50 {
                let players = vec![
                    Box::new(montecarlo::MontecarloPlayer::default()) as Box<dyn Player>,
                    Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
                    Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
                    Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
                ];
                let engine = Engine::new(players, Box::new(EmptyQueue::default()))?;

                stacks = engine.run(stacks.clone(), 1)?;
            }

            for i in 0..stacks.len() {
                if stacks[i] > old_stacks[i] {
                    wins[i] += 1;
                }
            }
        }

        for w in wins {
            print!("{} ", w as f64 / rounds as f64);
        }
        println!();

        Ok(())
    }

    #[test]
    fn run_from_started_game() {
        INIT.call_once(env_logger::init);

        let state = crate::core::state::GameState {
            round: Round::Preflop,
            community: vec![],
            players_bet: vec![1, 0, 0, 0],
            players_money: vec![0, 100, 100, 100],
            bet_amount: 1,
            players_all_in: vec![0],
            folded_players: vec![],
            num_active_players: 3,
            active_players: vec![1, 2, 3],
        };
        let players = vec![
            Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
            Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
            Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
            Box::new(dummy::DummyPlayer::default()) as Box<dyn Player>,
        ];
        let hand = (
            Card {
                suit: Suit::Club,
                value: Value::Ace,
            },
            Card {
                suit: Suit::Spade,
                value: Value::Ace,
            },
        );
        let player_idx = 1;

        let stacks = Engine::run_from_game_state(
            players,
            Box::new(TestQueue::default()),
            state,
            hand,
            player_idx,
        )
        .unwrap();
        println!("{stacks:?}");
    }
}
