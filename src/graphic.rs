extern crate sdl2;
use std::{collections::HashMap, path::Path, time::Duration};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, TextureQuery, WindowCanvas},
    ttf::Sdl2TtfContext,
    video::WindowContext,
};

use self::font::{FontParams, DEFAULT_FONT};

pub mod button;
pub mod community_renderer;
pub mod font;
pub mod player_render;
pub mod renderer;
pub mod self_render;
pub mod slider;
pub mod ui;
pub mod ui_component;

pub const TITLE: &str = "Rust game";
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

pub const TEXTURE_PATHS: [(&str, &str); 3] = [
    ("BACKGROUND","assets/vecteezy_poker-table-green-cloth-on-dark-background-vector-illustration_6325236.jpg"),
    ("CARD","assets/cards.png"),
    ("TITLE","assets/title-screen.jpg"),
];
pub const FONTS: [FontParams; 10] = [
    DEFAULT_FONT,
    DEFAULT_FONT.derive_size(24),
    DEFAULT_FONT.derive_size(36),
    DEFAULT_FONT.derive_size(48),
    DEFAULT_FONT.derive_size(72),
    DEFAULT_FONT.derive_size(128),
    DEFAULT_FONT.derive_color(Color::GREEN),
    DEFAULT_FONT
        .derive_size(72)
        .derive_color(Color::RGB(52, 128, 31)),
    DEFAULT_FONT.derive_size(128).derive_color(Color::RED),
    DEFAULT_FONT.derive_size(128).derive_color(Color::GREEN),
];

pub const CHARACTERS: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#%&'()*+,-./:;<=>?[\\]^_{|}~€$";

pub const START_DELAY: Duration = Duration::from_millis(1000);
pub const PLAY_DELAY: Duration = Duration::from_millis(1000);
pub const DEAL_DELAY: Duration = Duration::from_millis(300);
pub const SHOWDOWN_DELAY: Duration = Duration::from_millis(1000);

pub struct SDL2Graphics<'a> {
    pub canvas: WindowCanvas,
    ttf: Sdl2TtfContext,
    font_path: &'a Path,
    pub tex_cache: HashMap<&'a str, Texture<'a>>,
    pub font_cache: HashMap<(FontParams, char), Texture<'a>>,
}

impl<'a> SDL2Graphics<'a> {
    pub fn new(
        mut canvas: WindowCanvas,
        ttf: Sdl2TtfContext,
        font_path: &'a Path,
        creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let mut gfx = SDL2Graphics {
            canvas,
            ttf,
            font_path,
            tex_cache: HashMap::new(),
            font_cache: HashMap::new(),
        };

        for (name, path) in TEXTURE_PATHS {
            if let Ok(tex) = creator.load_texture(path) {
                gfx.tex_cache.insert(name, tex);
            }
        }

        gfx
    }

    pub fn start(&mut self, creator: &'a TextureCreator<WindowContext>) -> Result<(), String> {
        //Load fonts
        for params in FONTS {
            self.load_font(creator, params)?;
        }

        Ok(())
    }

    pub fn show(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) -> Result<(), String> {
        if let Some(bg) = self.tex_cache.get("BACKGROUND") {
            self.canvas.copy(bg, None, None)?;
        }

        Ok(())
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect)?;

        Ok(())
    }

    pub fn draw_string(
        &mut self,
        txt: &str,
        params: FontParams,
        mut p: Point,
        centered: bool,
    ) -> Result<(), String> {
        let mut width = 0;
        let mut height = 0;

        let mut chars = Vec::new();
        for c in txt.chars() {
            if let Some(char_tex) = self.font_cache.get(&(params, c)) {
                let (w, h) = texture_size(char_tex);
                width += w;
                height = h;
                chars.push(char_tex);
            }
        }

        if centered {
            p.x -= (width / 2) as i32;
            p.y -= (height / 2) as i32;
        }

        for tex in chars {
            let (w, h) = texture_size(tex);
            self.canvas.copy(tex, None, Rect::new(p.x, p.y, w, h))?;

            p.x += w as i32;
        }

        Ok(())
    }

    fn load_font(
        &mut self,
        creator: &'a TextureCreator<WindowContext>,
        params: FontParams,
    ) -> Result<(), String> {
        let font = self
            .ttf
            .load_font(self.font_path, params.size)
            .map_err(|e| e.to_string())?;

        for c in CHARACTERS.chars() {
            let surf = font
                .render(&c.to_string())
                .blended(params.color)
                .map_err(|e| e.to_string())?;

            let tex = creator
                .create_texture_from_surface(&surf)
                .map_err(|e| e.to_string())?;
            self.font_cache.insert((params, c), tex);
        }

        Ok(())
    }
}

pub fn texture_size(texture: &Texture) -> (u32, u32) {
    let TextureQuery { width, height, .. } = texture.query();
    (width, height)
}
