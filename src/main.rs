extern crate sdl2;

use std::path::Path;
use std::time::{Duration, Instant};

use poker::game::{Game, DEBUG};
use poker::graphic;
use poker::graphic::font::DEFAULT_FONT;
use poker::graphic::ui_component::{Drawable, EventReceiver};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

fn main() {
    env_logger::init();

    let sdl_context = sdl2::init().expect("Couldn't create SDL2 context");
    let ttf = sdl2::ttf::init().expect("Couldn't create text context");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't create video subsystem");
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)
        .expect("Couldn't initialize the image context");

    let font_path = Path::new("assets/RetroGaming.ttf");

    let window = video_subsystem
        .window(graphic::TITLE, graphic::WIDTH, graphic::HEIGHT)
        .fullscreen()
        .position_centered()
        .build()
        .expect("Couldn't create window");

    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Couldn't create canvas");

    //Load textures and construct gfx
    let creator = canvas.texture_creator();
    let mut gfx = graphic::SDL2Graphics::new(canvas, ttf, font_path, &creator);

    let mut game = Game::new(true);
    game.start();
    game.draw(&mut gfx).ok();

    let mut time = (0u128, 0i32, 0u128);
    let mut delta = Duration::ZERO;
    gfx.start(&creator).expect("Cannot load fonts in a cache");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Couldn't create the event loop");
    'running: loop {
        let t = Instant::now();

        game.early_update();

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
            game.handle_event(&event).ok();
        }

        //Internal structure update
        game.update(&delta);

        // Graphic update
        game.draw(&mut gfx).ok();
        if DEBUG.load(std::sync::atomic::Ordering::Relaxed) && game.is_running() {
            draw_time_elapsed(&mut gfx, time);
            gfx.show();
        }

        time.2 = t.elapsed().as_nanos();
        time.0 = time.0 * time.1 as u128 + time.2;
        time.1 += 1;
        time.0 /= time.1 as u128;
        delta = t.elapsed();
    }
}

fn draw_time_elapsed(gfx: &mut graphic::SDL2Graphics, time: (u128, i32, u128)) {
    let total_avg = 1.max(time.0 / 1000);
    let total = 1.max(time.2 / 1000);

    gfx.draw_string("DEBUG", DEFAULT_FONT, Point::new(10, 10), false)
        .unwrap();
    gfx.draw_string(
        &format!("Total time: {}us    AVG: {}us", total, total_avg),
        DEFAULT_FONT,
        Point::new(10, 40),
        false,
    )
    .unwrap();
    let string = format!(
        "FPS: {}             AVG: {}",
        1e6 as u128 / total,
        1e6 as u128 / total_avg
    );
    gfx.draw_string(&string, DEFAULT_FONT, Point::new(10, 70), false)
        .unwrap();

    //println!("{string}");
}
