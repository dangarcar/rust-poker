use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::graphic::ui;
use crate::graphic::SDL2Graphics;

pub mod self_controller;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    Playing,
    Start,
    Pause,
}

pub struct Game {
    ui: ui::UI,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            ui: ui::UI::new(),
            state: GameState::Start,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                Keycode::P => {
                    if self.state == GameState::Pause {
                        self.state = GameState::Playing;
                    } else if self.state == GameState::Playing {
                        self.state = GameState::Pause;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        self.ui.handle_event(event)?;
        Ok(())
    }

    pub fn update(&mut self) {}

    pub fn render(&self, gfx: &mut SDL2Graphics) -> Result<(), String> {
        self.ui.render(gfx)?;
        Ok(())
    }
}
