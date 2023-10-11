use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{self, TextureQuery},
    ttf::Font,
};

pub fn render_text(
    canvas: &mut render::WindowCanvas,
    font: &Font,
    txt: &str,
    color: Color,
    bounds: Rect,
) {
    let creator = canvas.texture_creator();
    let surf = font.render(txt).blended(color).unwrap();
    let txt_text = creator.create_texture_from_surface(surf).unwrap();
    canvas.copy(&txt_text, None, bounds).unwrap();
}

pub fn render_text_centered(
    canvas: &mut render::WindowCanvas,
    font: &Font,
    txt: &str,
    color: Color,
    c: Point,
) {
    let creator = canvas.texture_creator();
    let surf = font.render(txt).blended(color).unwrap();
    let txt_text = creator.create_texture_from_surface(surf).unwrap();
    let TextureQuery { width, height, .. } = txt_text.query();
    canvas
        .copy(&txt_text, None, Rect::from_center(c, width, height))
        .unwrap();
}

pub fn get_centered_rect(
    rect_width: u32,
    rect_height: u32,
    cons_width: u32,
    cons_height: u32,
) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (super::WIDTH as i32 - w) / 2;
    let cy = (super::HEIGHT as i32 - h) / 2;
    Rect::new(cx, cy, w as u32, h as u32)
}
