use std::sync::mpsc;
use std::thread;

use itertools::Itertools;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::action::mpsc_queue::MpscQueue;
use crate::core::action::GameMessage;
use crate::core::engine::Engine;
use crate::core::player::montecarlo::MontecarloPlayer;
use crate::core::player::myself::MyselfPlayer;
use crate::core::player::Player;
use crate::core::state::GameState;
use crate::game::player_state::PlayerAction;
use crate::graphic::ui;
use crate::graphic::ui_component::Drawable;
use crate::graphic::ui_component::EventReceiver;
use crate::graphic::SDL2Graphics;

use self::player_state::PlayerState;

pub mod player_state;
pub mod self_controller;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamePhase {
    Playing,
    Start,
    Pause,
}

pub struct Game {
    ui: ui::UI,
    phase: GamePhase,
    players: Option<Vec<PlayerState>>,
    myself: usize,
    game_rx: Option<mpsc::Receiver<GameMessage>>,
    player_tx: Option<mpsc::Sender<PlayerAction>>,
    game_state: Option<GameState>,
}

impl Game {
    pub fn new() -> Self {
        let ui = ui::UI::new();

        Game {
            ui,
            phase: GamePhase::Start,
            players: None,
            myself: 0,
            game_rx: None,
            player_tx: None,
            game_state: None,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
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
                _ => {}
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

    pub fn update(&mut self) {
        if let Some(rx) = &self.game_rx {
            match rx.try_recv() {
                Ok(msg) => self.update_player_state(msg),
                Err(_) => {}
            }
        }
    }

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.ui.draw(gfx)?;
        Ok(())
    }

    pub fn default_players(&mut self) {
        let mut rng = rand::thread_rng();
        self.players = Some(Vec::new());
        for _ in 0..8 {
            self.players.as_mut().unwrap().push(PlayerState {
                name: String::from_utf8(vec![rng.gen_range('1'..'z') as u8; 7]).unwrap(),
                bet: 0,
                cash: 1000,
                hand: None,
                can_raise: false,
                folded: false,
            });
        }

        self.myself = rng.gen_range(0..8);
    }

    pub fn start(&mut self) {
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
                    .map(|_| Box::new(MontecarloPlayer::default()) as Box<dyn Player>)
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

    fn update_player_state(&mut self, msg: GameMessage) {
        println!("{:?}", msg.action);

        //Set state to the engine's state at the end
        self.game_state = Some(msg.state);
    }
}
