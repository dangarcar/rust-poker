use sdl2::pixels;
use sdl2::ttf;

pub const NORMAL_16_BLACK: FontParams = FontParams {
    size: 16,
    style: ttf::FontStyle::NORMAL,
    color: pixels::Color::BLACK,
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
}
