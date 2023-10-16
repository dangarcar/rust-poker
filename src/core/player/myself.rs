use std::sync::mpsc;

use crate::{
    core::{error::EngineError, state::GameState},
    game::player_state,
};

use super::{Player, PlayerHand};

#[derive(Debug)]
pub struct MyselfPlayer {
    hand: Option<PlayerHand>,
    rx: mpsc::Receiver<player_state::PlayerAction>,
}

impl Player for MyselfPlayer {
    fn play(&mut self, state: &GameState, i: usize) -> Result<super::PlayerAction, EngineError> {
        let my_bet = state.players_bet[i];
        let diff = state.bet_amount - my_bet; //The amount to call

        let game_act = self.rx.recv().map_err(|_| EngineError::RecvMyselfError)?;

        Ok(match game_act {
            player_state::PlayerAction::Fold => super::PlayerAction::Fold,
            player_state::PlayerAction::Call => super::PlayerAction::Call(diff),
            player_state::PlayerAction::Raise(raised) => super::PlayerAction::Raise(diff + raised),
        })
    }

    fn cards(&self) -> Option<PlayerHand> {
        self.hand
    }

    fn give_cards(&mut self, hand: PlayerHand) {
        self.hand = Some(hand);
    }

    fn blind(&mut self, state: &GameState, i: usize) -> Result<super::PlayerAction, EngineError> {
        let my_bet = state.players_bet[i];
        let diff = state.bet_amount - my_bet; //The amount to call

        let game_act = self.rx.recv().map_err(|_| EngineError::RecvMyselfError)?;

        match game_act {
            player_state::PlayerAction::Fold => Ok(super::PlayerAction::Fold),
            player_state::PlayerAction::Call => Ok(super::PlayerAction::Call(diff)),
            player_state::PlayerAction::Raise(_) => Err(EngineError::NoRaiseAllowedError),
        }
    }
}

impl MyselfPlayer {
    pub fn new(rx: mpsc::Receiver<player_state::PlayerAction>) -> Self {
        MyselfPlayer { hand: None, rx }
    }
}
