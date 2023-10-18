use sdl2::pixels;
use sdl2::ttf;

pub const DEFAULT_FONT: FontParams = FontParams {
    size: 20,
    style: ttf::FontStyle::NORMAL,
    color: pixels::Color::WHITE,
};

#[derive(Debug, Clone, Copy)]
pub struct FontParams {
    pub size: u16,
    pub style: ttf::FontStyle,
    pub color: pixels::Color,
}

impl FontParams {
    pub fn new(size: u16, style: ttf::FontStyle, color: pixels::Color) -> Self {
        FontParams { size, style, color }
    }

    pub fn derive_color(&self, c: pixels::Color) -> FontParams {
        let mut f = *self;
        f.color = c;
        f
    }

    pub fn derive_size(&self, s: u16) -> FontParams {
        let mut f = *self;
        f.size = s;
        f
    }

    pub fn derive_style(&self, s: ttf::FontStyle) -> FontParams {
        let mut f = *self;
        f.style = s;
        f
    }
}
