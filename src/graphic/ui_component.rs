use super::SDL2Graphics;
use sdl2::event::Event;

pub trait EventReceiver<T> {
    fn handle_event(&mut self, event: &Event) -> T;
}

pub trait Drawable {
    fn draw(&self, gfx: &mut SDL2Graphics) -> Result<(), String>;
}
