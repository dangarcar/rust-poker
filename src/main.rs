use poker::core::deck;
use poker::graphic;

extern crate sdl2;

use poker::graphic::renderer::CARD_SPRITE_RATIO;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let context = graphic::GraphicContext::new();

    let mut canvas = context.canvas;
    let texture_creator = canvas.texture_creator();
    let texture = [
        texture_creator.load_texture(graphic::renderer::CARD_SPRITESHEET).unwrap()
    ];

    let mut event_pump = context.event_pump;

    let mut deck = deck::Deck::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(4, 125, 58));

        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        /*let mut x = 100;
        let mut y = 100;

        if deck.len() > 0 {
            for _ in 0..deck.len()-1 {
                x -= 1; y -= 1;
                let dst = Rect::new(x, y, 200, (200 as f32 *CARD_SPRITE_RATIO) as u32);
                graphic::renderer::render_card(&mut canvas, &texture[0], &None, dst).unwrap();
            }

            let card = deck.take();
            let dst = Rect::new(500, 100, 200, (200 as f32 *CARD_SPRITE_RATIO) as u32);
            graphic::renderer::render_card(&mut canvas, &texture[0], &card, dst).unwrap();
        }*/

        canvas.present();

        //thread::sleep(time::Duration::from_millis(250));
    }
}