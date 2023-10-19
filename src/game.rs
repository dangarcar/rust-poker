use std::sync::mpsc;
use std::thread;

use itertools::Itertools;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

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
use crate::graphic::font::DEFAULT_FONT;
use crate::graphic::ui;
use crate::graphic::ui_component::Drawable;
use crate::graphic::ui_component::EventReceiver;
use crate::graphic::SDL2Graphics;
use crate::graphic::HEIGHT;
use crate::graphic::WIDTH;

use self::player_state::PlayerState;

pub mod player_state;
pub mod self_controller;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamePhase {
    #[default]
    Start,
    Playing,
    Pause,
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

    pub turn: usize,
}

impl Game {
    //TODO: New players that aren't default
    pub fn new(default_players: bool) -> Self {
        let mut game = Game::default();

        if default_players {
            game.default_players();
        }

        game
    }

    pub fn early_update(&mut self) {
        if let Some(state) = &self.game_state {
            self.ui.player_controller.early_update(state);
        }
    }

    pub fn update(&mut self) {
        if let Some(rx) = &self.game_rx {
            if let Ok(msg) = rx.try_recv() {
                self.update_player_state(msg);
            }
        }

        if let Some(player_states) = &self.players {
            self.ui.update_states(player_states, self.myself);
        }
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

    fn update_player_state(&mut self, msg: GameMessage) {
        let state = msg.state;

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
                        self.turn = state.active_players[0];
                        players[self.turn].turn = true;
                    }
                }
                GameAction::DealCommunity { card } => {
                    self.ui.community.add_card(card);
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

                    update_turn(&mut self.turn, &state, i, players);
                }
                GameAction::PlayedFolded { action, i } => {
                    match action {
                        player::PlayerAction::Fold => players[i].folded = true,
                        _ => panic!("A bet is not a fold"),
                    }
                    update_turn(&mut self.turn, &state, i, players);
                }
                GameAction::ErroredPlay { error, i } => {
                    println!("{error}");
                    update_turn(&mut self.turn, &state, i, players);
                }
                GameAction::ShowdownHand { hand, rank, i } => {
                    players[i].hand = Some(hand);
                    players[i].rank = Some(rank);
                    update_turn(&mut self.turn, &state, i, players);
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
                cash: 1000,
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
                Keycode::A => println!("A pressed"),
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

impl Drawable for Game {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.ui.draw(gfx)?;

        if let GamePhase::Ended(rank, i, pot) = self.phase {
            gfx.draw_rect(Rect::new(0, 0, WIDTH, HEIGHT), Color::RGBA(0, 0, 0, 200))?;

            if let Some(players) = &self.players {
                if i != self.myself {
                    gfx.draw_string(
                        "GAME OVER",
                        DEFAULT_FONT.derive_size(128).derive_color(Color::RED),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 - 50),
                        true,
                    )?;
                    gfx.draw_string(
                        &format!("Player {} won {}€", players[i].name, pot),
                        DEFAULT_FONT.derive_size(48),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 + 50),
                        true,
                    )?;
                } else {
                    gfx.draw_string(
                        "YOU WON",
                        DEFAULT_FONT.derive_size(128).derive_color(Color::GREEN),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 - 50),
                        true,
                    )?;
                    gfx.draw_string(
                        &format!("You have won {}€", pot),
                        DEFAULT_FONT.derive_size(48),
                        Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 + 50),
                        true,
                    )?;
                }

                gfx.draw_string(
                    &format!("Rank: {:?}", rank),
                    DEFAULT_FONT.derive_size(48),
                    Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 + 120),
                    true,
                )?;
            }
        }

        Ok(())
    }
}

fn update_turn(turn: &mut usize, state: &GameState, i: usize, players: &mut [PlayerState]) {
    players[i].turn = false;
    *turn = i + 1;

    while *turn < players.len() {
        if !state.folded_players.contains(turn) && !state.players_all_in.contains(turn) {
            players[*turn].turn = true;    
            break;
        }
        *turn += 1;
    }
}