extern crate sdl2;
use sdl2::{render, image};

const TITLE: &'static str = "Rust game";
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

pub mod renderer;

pub struct GraphicContext {
    _sdl_context: sdl2::Sdl,
    _image_context: image::Sdl2ImageContext,
    pub canvas: render::WindowCanvas,
    pub event_pump: sdl2::EventPump,
}

impl GraphicContext {
    pub fn new() -> Self {
        let sdl_context = sdl2::init()
            .expect("Couldn't create SDL2 context");
        let video_subsystem = sdl_context.video()
            .expect("Couldn't create video subsystem");
        let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)
            .expect("Couldn't initialize the image context");

        let window = video_subsystem.window(TITLE, WIDTH, HEIGHT)
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

        let event_pump = sdl_context.event_pump()
            .expect("Couldn't create the event pump");

        GraphicContext { 
            _sdl_context: sdl_context, 
            _image_context,
            canvas, 
            event_pump 
        }
    }
}
