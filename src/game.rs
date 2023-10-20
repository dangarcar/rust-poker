use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use itertools::Itertools;
use rand::Rng;
use rand::thread_rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::action::game_action::GameAction;
use crate::core::action::mpsc_queue::MpscQueue;
use crate::core::action::GameMessage;
use crate::core::engine::Engine;
use crate::core::player;
use crate::core::player::montecarlo::MontecarloPlayer;
use crate::core::player::myself::MyselfPlayer;
use crate::core::player::Player;
use crate::core::rank::Rank;
use crate::core::state::GameState;
use crate::core::state::Round;
use crate::game::player_state::PlayerAction;
use crate::graphic::ui_component::EventReceiver;
use crate::graphic::{DEAL_DELAY, PLAY_DELAY, START_DELAY, SHOWDOWN_DELAY, ui};

use self::player_state::PlayerState;

pub mod player_state;
pub mod self_controller;
pub mod game_render;

pub static DEBUG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamePhase {
    #[default]
    Start,
    Playing,
    Pause,
    Showdown(usize),
    Ended(Rank, usize, i32),
}

#[derive(Default)]
pub struct Game {
    ui: ui::UI,
    phase: GamePhase,
    players: Option<Vec<PlayerState>>,
    myself: usize,
    game_rx: Option<mpsc::Receiver<GameMessage>>,
    player_tx: Option<mpsc::Sender<PlayerAction>>,
    game_state: Option<GameState>,

    pub delay: Duration,
    turn: usize,
}

impl Game {
    //TODO: New players that aren't default
    pub fn new(default_players: bool) -> Self {
        let mut game = Game::default();
        game.turn = usize::MAX;

        if default_players {
            game.default_players();
        }

        game
    }

    pub fn early_update(&mut self) {
        if let Some(state) = &self.game_state {
            if !state.folded_players.contains(&self.myself) && !state.players_all_in.contains(&self.myself) {
                self.ui.player_controller.early_update(state);
            }
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        if !self.delay.is_zero() {
            if self.delay <= *delta {
                self.delay = Duration::ZERO;
                self.on_delay_ended();
            } else {
                self.delay -= *delta;
            }

            return;
        }

        match self.phase {
            GamePhase::Start => self.phase = GamePhase::Playing,
            GamePhase::Playing => {
                if let Some(rx) = &self.game_rx {
                    if let Ok(msg) = rx.try_recv() {
                        self.update_player_state(msg);
                    }
                }

                if let Some(player_states) = &mut self.players {
                    self.ui.update_states(player_states, self.myself);
                }

                if let Some(state) = &self.game_state {
                    self.ui.community.pot = state.players_bet.iter().sum();
                }
            }
            GamePhase::Showdown(..) => {}
            GamePhase::Pause => {}
            GamePhase::Ended(..) => {}
        }
    }

    pub fn start(&mut self) {
        self.delay = START_DELAY;

        let (game_tx, game_rx) = mpsc::channel();
        self.game_rx = Some(game_rx);

        let (player_tx, player_rx) = mpsc::channel();
        self.player_tx = Some(player_tx);

        let this = self.myself;

        //Start engine thread
        if let Some(player_states) = self.players.clone() {
            thread::spawn(move || {
                let queue = Box::new(MpscQueue::new(game_tx));
                let mut players = player_states
                    .iter()
                    .map(|_| Box::<MontecarloPlayer>::default() as Box<dyn Player>)
                    .collect_vec();
                players[this] = Box::new(MyselfPlayer::new(player_rx));

                let engine = Engine::new(players, queue).expect("Cannot create poker engine");

                let players_money = player_states.iter().map(|p| p.cash).collect_vec();
                engine.run(players_money, 1).unwrap();
            });
        }

        //Start UI
        if let Some(player_states) = &self.players {
            self.ui
                .start(player_states, self.myself)
                .expect("Couldn't start the UI");
        }
    }

    fn on_delay_ended(&mut self) {
        if let GamePhase::Showdown(..) = self.phase {
            self.phase = GamePhase::Playing;
        }
        else if let (Some(players), Some(state))= (&mut self.players, &self.game_state) {
            update_turn(&mut self.turn, state, players);
        }
    }

    fn update_player_state(&mut self, msg: GameMessage) {
        let state = msg.state;
        let mut rng = thread_rng();

        if let Some(players) = &mut self.players {
            match msg.action {
                GameAction::DealStartHand { hand, i } => {
                    if i == self.myself {
                        players[i].hand = Some(hand);
                    }
                }
                GameAction::RoundChanged { round } => {
                    for p in players.iter_mut() {
                        p.can_raise = round != Round::Preflop;
                    }
                    if round >= Round::Preflop && round < Round::Complete {
                        self.turn = 0;
                        while self.turn < players.len() {
                            if !state.folded_players.contains(&self.turn) && !state.players_all_in.contains(&self.turn) {
                                players[self.turn].turn = true;
                                break;
                            }
                            self.turn += 1;
                        }
                    }
                }
                GameAction::DealCommunity { card } => {
                    self.ui.community.add_card(card);
                    self.delay = DEAL_DELAY;
                }
                GameAction::PlayedBet { action, i, all_in } => {
                    match action {
                        player::PlayerAction::Raise(a) | player::PlayerAction::Call(a) => {
                            players[i].cash -= a;
                            players[i].bet += a;
                            players[i].all_in = all_in;
                        }
                        player::PlayerAction::Fold => panic!("A fold is not a bet"),
                    }

                    self.turn = i;
                    self.delay = PLAY_DELAY.mul_f32(rng.gen_range(0.5..=1.0));
                }
                GameAction::PlayedFolded { action, i } => {
                    match action {
                        player::PlayerAction::Fold => players[i].folded = true,
                        _ => panic!("A bet is not a fold"),
                    }
                    
                    self.turn = i;
                    self.delay = PLAY_DELAY.mul_f32(rng.gen_range(0.5..=1.0));
                }
                GameAction::ErroredPlay { error, i } => {
                    println!("{error}");

                    self.turn = i;
                    self.delay = PLAY_DELAY.mul_f32(rng.gen_range(0.5..=1.0));
                }
                GameAction::ShowdownHand { hand, rank, i } => {
                    players[i].hand = Some(hand);
                    players[i].rank = Some(rank);

                    self.turn = i;
                    update_turn(&mut self.turn, &state, players);
                    self.phase = GamePhase::Showdown(i);
                    self.delay = SHOWDOWN_DELAY;
                }
                GameAction::WinGame { rank, i, pot } => {
                    players[i].turn = true;
                    self.phase = GamePhase::Ended(rank, i, pot);
                }
            };
        }

        //Set state to the engine's state at the end
        self.game_state = Some(state);
    }

    pub fn default_players(&mut self) {
        let mut rng = rand::thread_rng();
        let max_p = 8;
        self.players = Some(Vec::new());
        for i in 0..max_p {
            self.players.as_mut().unwrap().push(PlayerState {
                name: format!("Player{}", i + 1),
                bet: 0,
                cash: 100000,
                hand: None,
                rank: None,
                can_raise: false,
                folded: false,
                all_in: false,
                turn: false,
            });
        }

        self.myself = rng.gen_range(0..max_p);
        self.players.as_mut().unwrap()[self.myself].name = "Me".to_string();
    }

    pub fn is_running(&self) -> bool {
        self.phase == GamePhase::Playing
    }
}

impl EventReceiver<Result<(), String>> for Game {
    fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        if let GamePhase::Ended(..) = self.phase {
            return Ok(());
        }

        #[allow(clippy::single_match)]
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                Keycode::P => {
                    if self.phase == GamePhase::Pause {
                        self.phase = GamePhase::Playing;
                    } else if self.phase == GamePhase::Playing {
                        self.phase = GamePhase::Pause;
                    }
                }
                Keycode::D => {
                    let d = !DEBUG.load(std::sync::atomic::Ordering::Relaxed);
                    DEBUG.store(d, std::sync::atomic::Ordering::Relaxed);
                },
                _ => (),
            },
            _ => {}
        }

        if let Some(act) = self.ui.handle_event(event)? {
            if let Some(tx) = &self.player_tx {
                tx.send(act).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}

fn update_turn(turn: &mut usize, state: &GameState, players: &mut [PlayerState]) {
    players[*turn].turn = false;
    if *turn < players.len() { *turn += 1; }

    while *turn < players.len() {
        if !state.folded_players.contains(turn) && !state.players_all_in.contains(turn) {
            players[*turn].turn = true;
            break;
        }
        *turn += 1;
    }
}
