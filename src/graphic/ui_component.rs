use sdl2::event::Event;
use super::SDL2Graphics;

pub trait EventReceiver {
    fn handle_event(&mut self, event: &Event);
}

pub trait Drawable {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String>;
}