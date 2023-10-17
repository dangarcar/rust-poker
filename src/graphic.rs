extern crate sdl2;
use std::{collections::HashMap, path::Path};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, TextureQuery, WindowCanvas},
    ttf::Sdl2TtfContext,
    video::WindowContext,
};

use self::font::FontParams;

pub mod button;
pub mod community_renderer;
pub mod font;
pub mod player_render;
pub mod renderer;
pub mod self_render;
pub mod slider;
pub mod ui;
pub mod ui_component;

pub const TITLE: &'static str = "Rust game";
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

pub const TEXTURE_PATHS: [(&'static str, &'static str); 2] = [
    ("BACKGROUND","assets/vecteezy_poker-table-green-cloth-on-dark-background-vector-illustration_6325236.jpg"),
    ("CARD","assets/cards.png"),
];

pub struct SDL2Graphics<'a> {
    pub canvas: WindowCanvas,
    ttf: Sdl2TtfContext,
    font_path: &'a Path,
    pub tex_cache: HashMap<&'a str, Texture<'a>>,
}

impl<'a> SDL2Graphics<'a> {
    pub fn new(mut canvas: WindowCanvas, ttf: Sdl2TtfContext, font_path: &'a Path) -> Self {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        SDL2Graphics {
            canvas,
            ttf,
            font_path,
            tex_cache: HashMap::new(),
        }
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

    pub fn draw_string(&mut self, txt: &str, params: FontParams, p: Point, centered: bool) {
        if txt.is_empty() {
            return;
        }

        let texture_creator = self.canvas.texture_creator();
        let texture = self.str_to_texture(txt, params, &texture_creator);

        let (w, h) = texture_size(&texture);
        let bounds = if centered {
            Rect::from_center(p, w, h)
        } else {
            Rect::new(p.x, p.y, w, h)
        };

        self.canvas
            .copy(&texture, None, bounds)
            .expect("Could not write the string");
    }

    pub fn string_size(&self, txt: &str, params: FontParams) -> (u32, u32) {
        let tex_creator = self.canvas.texture_creator();
        let tex = self.str_to_texture(txt, params, &tex_creator);
        texture_size(&tex)
    }

    fn str_to_texture(
        &self,
        txt: &str,
        params: FontParams,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture<'a> {
        let font = self
            .ttf
            .load_font(self.font_path, params.size)
            .expect("Error while loading font");
        let surf = font
            .render(txt)
            .blended(params.color)
            .expect("Error while rendering text to surface");

        texture_creator
            .create_texture_from_surface(&surf)
            .expect("Error while converting from surface to texture")
    }
}

pub fn texture_size(texture: &Texture) -> (u32, u32) {
    let TextureQuery { width, height, .. } = texture.query();
    (width, height)
}
