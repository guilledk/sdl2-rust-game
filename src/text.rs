extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use self::sdl2_ttf::Font;
use self::sdl2_ttf::Sdl2TtfContext;

pub struct TextRenderer<'a> {
    context: &'a Sdl2TtfContext,
    default_font: Font,
    default_glyphs: Texture,
}

impl<'a> TextRenderer<'a> {
    pub fn init(ctx: &'a Sdl2TtfContext, renderer: &mut Renderer, deff: &Path) -> TextRenderer<'a> {
        let tmpfont = ctx.load_font(deff, 14).unwrap();
        TextRenderer {
            context: ctx,
            default_font: ctx.load_font(deff, 14).unwrap(),
            default_glyphs: renderer.create_texture_from_surface(
                tmpfont.render("abcdefghijklmnopkrstuvwxyz0123456789")
                .blended(Color::RGBA(255, 255, 255, 255)).unwrap()
            ).unwrap()
        }
    }
    pub fn draw_string(&self, text: String, renderer: &mut Renderer, x: i32, y: i32) {
        let height = self.default_font.height();
        let mut xpos = 0;
        for c in text.chars() {
            let cur_w = self.default_font.find_glyph_metrics(c).unwrap().maxx;
            let dest = Rect::new(xpos + x,y,cur_w as u32,height as u32);
            let src = Rect::new((c as u32 - 97) as i32 * cur_w,0,cur_w as u32,height as u32);
            renderer.copy(&self.default_glyphs, Some(src), Some(dest));
            xpos += cur_w;
        }
    }
}
