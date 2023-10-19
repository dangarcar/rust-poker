use sdl2::{pixels::Color, rect::Rect};

use crate::{
    core::state::GameState,
    graphic::{
        button::{Button, ButtonState},
        self_render::{CALL_COLOR, FOLD_COLOR, RAISE_COLOR},
        slider::Slider,
        ui_component::EventReceiver,
        HEIGHT, WIDTH,
    },
};

use super::player_state::{PlayerAction, PlayerState};

pub struct SelfController {
    pub bounds: Rect,

    pub raise_btn: Button,
    pub call_btn: Button,
    pub fold_btn: Button,
    pub slider: Slider,
    pub image_bounds: Rect,

    pub state: PlayerState,
    pub diff: i32,
}

impl EventReceiver<Option<PlayerAction>> for SelfController {
    fn handle_event(&mut self, event: &sdl2::event::Event) -> Option<PlayerAction> {
        self.slider.handle_event(event);

        let raise =
            self.raise_btn.handle_event(event) == ButtonState::Pressed && self.state.can_raise;
        let call = self.call_btn.handle_event(event) == ButtonState::Pressed;
        let fold = self.fold_btn.handle_event(event) == ButtonState::Pressed;

        if !self.state.can_raise {
            self.raise_btn.set_inactive();
            self.slider.set_inactive();
        }

        if self.state.folded {
            self.raise_btn.set_inactive();
            self.call_btn.set_inactive();
            self.fold_btn.set_inactive();
            return None;
        }

        if raise {
            let s = Some(PlayerAction::Raise(self.to_raise()));
            self.slider.reset();
            s
        } else if call {
            Some(PlayerAction::Call)
        } else if fold {
            Some(PlayerAction::Fold)
        } else {
            None
        }
    }
}

impl Default for SelfController {
    fn default() -> Self {
        let h = 230;
        let y = HEIGHT as i32 - h;
        let bounds = Rect::new(0, y, WIDTH, h as u32);

        let raise_btn = Button::new(
            "RAISE".to_string(),
            Rect::new(bounds.right() - 750, y + 100, 200, 100),
            RAISE_COLOR,
        );
        let fold_btn = Button::new(
            "FOLD".to_string(),
            Rect::new(bounds.right() - 250, y + 100, 200, 100),
            FOLD_COLOR,
        );
        let call_btn = Button::new(
            "CALL".to_string(),
            Rect::new(bounds.right() - 500, y + 100, 200, 100),
            CALL_COLOR,
        );
        let slider = Slider::new(
            Rect::new(bounds.right() - 750, y + 50, 700, 30),
            Color::BLUE,
            Color::GRAY,
            Color::BLACK,
        );
        let image_bounds = Rect::new(50, y + 50, 150, 150);

        SelfController {
            bounds,
            raise_btn,
            call_btn,
            fold_btn,
            slider,
            image_bounds,
            state: Default::default(),
            diff: 0,
        }
    }
}

impl SelfController {
    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    pub fn to_raise(&self) -> i32 {
        (self.slider.value() * self.state.cash as f32) as i32
    }

    pub fn early_update(&mut self, state: &GameState) {
        if self.state.can_raise {
            self.raise_btn.set_active();
        }
        self.diff = state.bet_amount - self.state.bet;
    }
}
