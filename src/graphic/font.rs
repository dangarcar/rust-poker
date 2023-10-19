use sdl2::pixels;

pub const DEFAULT_FONT: FontParams = FontParams {
    size: 20,
    color: pixels::Color::WHITE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontParams {
    pub size: u16,
    pub color: pixels::Color,
}

impl FontParams {
    pub fn new(size: u16, color: pixels::Color) -> Self {
        FontParams { size, color }
    }

    pub const fn derive_color(&self, c: pixels::Color) -> FontParams {
        let mut f = *self;
        f.color = c;
        f
    }

    pub const fn derive_size(&self, s: u16) -> FontParams {
        let mut f = *self;
        f.size = s;
        f
    }
}
