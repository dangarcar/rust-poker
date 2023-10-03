use crate::core::card::Card;

use super::error::EngineError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Round {
    Starting,
    Preflop, //Bet
    Flop, //Bet
    Turn, //Bet
    River, //Bet
    Showdown,
    Complete,
}

impl Round {
    pub fn next(&self) -> Self {
        match *self {
            Round::Starting => Round::Preflop,
            Round::Preflop => Round::Flop,
            Round::Flop => Round::Turn,
            Round::Turn => Round::River,
            Round::River => Round::Showdown,
            Round::Showdown => Round::Complete,
            Round::Complete => Round::Complete,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub round: Round,
    pub community: Vec<Card>,

    pub players_bet: Vec<i32>,
    pub players_money: Vec<i32>,
    
    pub bet_amount: i32,
    pub players_all_in: Vec<usize>,

    pub folded_players: Vec<usize>,

    pub num_active_players: i32,
    pub active_players: Vec<usize>,
}

impl GameState {
    pub fn bet(&mut self, amount: i32, player_idx: usize, all_in: bool) -> Result<(), EngineError> {
        self.validate_bet(amount, player_idx, all_in)?;

        if all_in {
            self.players_all_in.push(player_idx);
            self.num_active_players -= 1;
            
            self.players_bet[player_idx] += self.players_money[player_idx];
            self.players_money[player_idx] = 0;

            self.remove_inactive_players();
        }
        else {
            self.players_bet[player_idx] += amount;
            self.players_money[player_idx] -= amount;
        }

        self.bet_amount = *self.players_bet.iter().max().unwrap();

        Ok(())
    }

    fn validate_bet(&self, amount: i32, player_idx: usize, all_in: bool) -> Result<(), EngineError> {
        if self.players_money[player_idx] < amount && !all_in {
            Err(EngineError::NotEnoughMoney)
        }
        else {
            Ok(())
        }
    }

    pub fn award(&mut self, player: usize) -> i32 {
        let pot: i32 = self.players_bet.iter().sum();

        self.players_money[player] += pot;

        self.players_bet.clear();

        pot
    }

    pub fn remove_inactive_players(&mut self) {
        self.active_players.retain(|i| !self.folded_players.contains(i) && !self.players_all_in.contains(i));
        self.num_active_players = self.active_players.len() as i32;
    }
}