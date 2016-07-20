extern crate sdl2;

use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

pub struct GameState<'a> {
    pub keys: [bool; 5],
    pub logo: Sprite<'a>,
}

pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl RectF {
    pub fn to_sdl_rect(&self) -> Rect {
        Rect::new(
            self.x.round() as i32,
            self.y.round() as i32,
            self.w.round() as u32,
            self.h.round() as u32
        )
    }
}

pub struct Sprite<'a> {

    pub img: &'a Texture,
    pub rect: RectF,

}

impl<'a> Sprite<'a> {

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.copy(&self.img, None, Some(self.rect.to_sdl_rect()));
    }

}

/*
 *keys:
 *  0: Escape
 *  1: w
 *  2: a
 *  3: s
 *  4: d
 */
