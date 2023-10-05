extern crate sdl2;

use std::path::Path;

use poker::game::Game;
use poker::graphic;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    env_logger::init();

    let sdl_context = sdl2::init().expect("Couldn't create SDL2 context");
    let ttf = sdl2::ttf::init().expect("Couldn't create text context");
    let video_subsystem = sdl_context.video().expect("Couldn't create video subsystem");
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).expect("Couldn't initialize the image context");

    let font_path = Path::new("assets/Roboto-Regular.ttf");

    let window = video_subsystem.window(graphic::TITLE, graphic::WIDTH, graphic::HEIGHT)
        //.fullscreen_desktop()
        .fullscreen()
        .position_centered()
        .build()
        .expect("Couldn't create window");
    let canvas = window.into_canvas()
        .accelerated()
        //.present_vsync()
        .build()
        .expect("Couldn't create canvas");

    let mut gfx = graphic::SDL2Graphics::from(canvas, ttf, font_path);

    let mut game = Game::new();

    let mut event_pump = sdl_context.event_pump().expect("Couldn't create the event loop");
    'running: loop {
        //Event update
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
            game.handle_event(&event);
        }

        //Internal structure update
        game.update();

        // Graphic update
        gfx.clear();
        game.render(&mut gfx).ok();
        gfx.show();
    }
}
