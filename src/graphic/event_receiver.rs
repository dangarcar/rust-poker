use sdl2::event::Event;

pub trait EventReceiver {
    fn handle_event(&mut self, event: &Event);
}