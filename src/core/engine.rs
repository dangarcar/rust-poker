use crate::core::card::*;
use crate::core::hand::Hand;
use crate::core::rank::Rank;
use crate::core::rank::Rankable;
use crate::player::*;
use crate::core::deck::*;
use crate::core::state::*;

use super::EngineError;

use log::debug;

#[derive(Debug)]
pub struct Engine {
    deck: Deck,
    pub state: GameState,
    pub players_hands: Vec<(Card, Card)>,
    pub players: Vec<Box<dyn Player>>,
}

impl Engine {
    pub fn new(players: Vec<Box<dyn Player>>) -> Result<Self, EngineError> {
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
            deck, 
            state, 
            players_hands,
            players,
        })
    }

    pub fn run(mut self, players_money: Vec<i32>, blind: i32) -> Result<Vec<i32>, EngineError> {
        if self.players.len() != players_money.len() { return Err(EngineError::BadGameError); }
        else { self.state.players_money = players_money; }

        if !blind > 0 { return Err(EngineError::SmallBlindError); }
        else { self.state.bet_amount = blind; }

        loop {
            match self.state.round {
                Round::Starting => self.start()?,
                Round::Preflop => self.preflop()?,
                Round::Flop => self.flop()?,
                Round::Turn => self.turn()?,
                Round::River => self.river()?,
                Round::Showdown => self.showdown()?,

                Round::Complete => return Ok(self.state.players_money)
            };
        }
    }

    pub fn run_from_game_state(state: GameState, players: Vec<Box<dyn Player>>, hand: PlayerHand, player_idx: usize) -> Result<Vec<i32>, EngineError> {
        let deck = Deck::new_without_cards(&[hand.0, hand.1]);
        let players_hands = Vec::new();

        let mut engine = Engine { 
            deck, 
            state, 
            players_hands,
            players,
        }; 

        for i in 0..engine.players.len() {
            if i == player_idx {
                engine.players_hands.push(hand);
            } else {
                let h = (
                    engine.deck.take().ok_or(EngineError::BadDeckError)?, 
                    engine.deck.take().ok_or(EngineError::BadDeckError)?
                );
                engine.players_hands.push(h);
                engine.players[i].give_cards(h);
            }
        }

        loop {
            match engine.state.round {
                Round::Starting => engine.start()?,
                Round::Preflop => engine.preflop()?,
                Round::Flop => engine.flop()?,
                Round::Turn => engine.turn()?,
                Round::River => engine.river()?,
                Round::Showdown => engine.showdown()?,

                Round::Complete => return Ok(engine.state.players_money)
            };
        }
    }

    fn start(&mut self) -> Result<(), EngineError> {
        for p in self.players.iter_mut() {
            self.state.players_bet.push(0);

            let hand = (
                self.deck.take().ok_or(EngineError::BadDeckError)?, 
                self.deck.take().ok_or(EngineError::BadDeckError)?
            );
            self.players_hands.push(hand);
            p.give_cards(hand);
        }

        self.state.round = self.state.round.advance();
        Ok(())
    }

    fn preflop(&mut self) -> Result<(), EngineError> {
        debug!("Preflop round");
        //let mut folded_players = Vec::new();
        
        for i in self.state.active_players.clone() {
            //If there's only one player, there's no need to play
            if self.state.num_active_players <= 1 {
                break;
            }
            
            let action = self.players[i].blind(&self.state, i);

            match action {
                Ok(PlayerAction::Call(amount)) => {
                    let all_in = self.state.bet_amount >= self.state.players_money[i] + self.state.players_bet[i];
                    self.state.bet(amount, i, all_in)?;
                }
                Ok(PlayerAction::Fold) => {
                    self.state.folded_players.push(i);
                    self.state.remove_inactive_players();
                }
                Ok(PlayerAction::Raise(_)) => {
                    self.state.folded_players.push(i);
                    self.state.remove_inactive_players();
                    return Err(EngineError::NoRaiseAllowedError);
                }
                Err(e) => {
                    println!("{e}");

                    self.state.folded_players.push(i);
                    self.state.remove_inactive_players();
                }
            }
        }

        self.state.round = self.state.round.advance();
        Ok(())
    }

    fn flop(&mut self) -> Result<(), EngineError> {
        debug!("Flop round");
        
        self.deal_community(3)?;
        self.betting_round()?;

        self.state.round = self.state.round.advance();
        Ok(())
    }

    fn turn(&mut self) -> Result<(), EngineError> {
        debug!("Turn round");
        
        self.deal_community(1)?;
        self.betting_round()?;

        self.state.round = self.state.round.advance();
        Ok(())
    }

    fn river(&mut self) -> Result<(), EngineError> {
        debug!("River round");
        
        self.deal_community(1)?;
        self.betting_round()?;

        self.state.round = self.state.round.advance();
        Ok(())
    }

    fn showdown(&mut self) -> Result<(), EngineError> {
        debug!("Showdown round");
        
        let mut winner = (Rank::HighCard(Value::Two), usize::MAX); //The worst possible rank
        let mut pos = self.state.active_players.clone();
        pos.append(&mut self.state.players_all_in);

        println!("{}", pos.len());

        for i in pos {
            let mut hand = Hand::new_from_cards(self.state.community.clone());
            hand.push(self.players_hands[i].0);
            hand.push(self.players_hands[i].1);

            let rank = hand.rank()?;
            if winner.0 < rank {
                winner.1 = i;
                winner.0 = rank;
            }
        }

        self.state.award(winner.1);

        self.state.round = self.state.round.advance();
        Ok(())
    }

    fn betting_round(&mut self) -> Result<(), EngineError> {
        debug!("Betting round");

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
                        let all_in = self.state.bet_amount == self.state.players_money[i] + self.state.players_bet[i];
                        self.state.bet(amount, i, all_in)?;

                        raising = true;
                    }
                    Ok(PlayerAction::Call(amount)) => {
                        let all_in = self.state.bet_amount >= self.state.players_money[i] + self.state.players_bet[i];
                        self.state.bet(amount, i, all_in)?;
                    }
                    Ok(PlayerAction::Fold) => {
                        self.state.folded_players.push(i);
                        self.state.remove_inactive_players();
                    }
                    Err(e) => {
                        println!("{e}");

                        self.state.folded_players.push(i);
                        self.state.remove_inactive_players();
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
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{player, core::EngineError};

    use super::Engine;

    #[test]
    fn debug_engine() -> Result<(), EngineError>{
        env_logger::init();

        let players = vec![
            Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
            Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
            Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
            Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
        ];

        let old_stacks = vec![100, 100, 100, 100];

        let engine = Engine::new(players)?;

        let new_stacks =  engine.run(old_stacks.clone(), 1)?;

        println!("{:?}", old_stacks);
        println!("{:?}", new_stacks);

        Ok(())
    }

    #[test]
    #[ignore = "Takes ages for running"]
    fn loop_dummies() -> Result<(), EngineError> {
        env_logger::init();

        let rounds = 10000;
        let initial_money = 10000;

        let mut stacks = vec![initial_money; 8];
        let mut old_stacks = stacks.clone();

        for i in 0..rounds {
            let players = vec![
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
                Box::new(player::dummy::DummyPlayer::default()) as Box<dyn player::Player>,
            ];
            let engine = Engine::new(players)?;

            stacks =  engine.run(stacks.clone(), 1)?;
            println!("- {i} {:?} {:?}", old_stacks, stacks);
            old_stacks = stacks.clone();
        }

        Ok(())
    }
}