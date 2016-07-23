extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use self::sdl2_ttf::Font;
use self::sdl2_ttf::Sdl2TtfContext;

pub struct GlyphAtlas {
    glyphs: Texture,
    font: Font,
    height: u32,
}

const ATLAS: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz";

impl GlyphAtlas {

    pub fn create_glyph_atlas(font: &Font) -> Surface {
        let mut atlas = Surface::new(32 * 91,font.height() as u32,PixelFormatEnum::RGBA8888).unwrap();
        let mut index = 0;
        for c in ATLAS.chars() {
            font.render_char(c).blended(Color::RGBA(255, 255, 255, 255)).unwrap()
            .blit(None, atlas.as_mut(), Some(Rect::new(
                32 * index,
                0,
                font.find_glyph_metrics(c).unwrap().maxx as u32,
                font.height() as u32
            )));
            index += 1;
        }
        atlas
    }

    pub fn from_path(path: &Path, ctx: &Sdl2TtfContext, renderer: &Renderer, size: u16) -> GlyphAtlas {
        let tmpfont = ctx.load_font(path, size).unwrap();
        let ret = GlyphAtlas {
            glyphs: renderer.create_texture_from_surface(
                GlyphAtlas::create_glyph_atlas(&tmpfont)
            ).unwrap(),
            font: ctx.load_font(path, size).unwrap(),
            height: tmpfont.height() as u32,
        };
        return ret;
    }

    pub fn draw_string(&self, text: String, renderer: &mut Renderer, x: i32, y: i32) {
        let mut xpos: i32 = 0;
        for c in text.chars() {
            if c == ' ' { xpos += 11; continue }
            if c as u32 > 122 { panic!("Can't print '{}'", c) }
            let cur_width = self.font.find_glyph_metrics(c).unwrap().maxx as u32;
            let dst = Rect::new(xpos + x,y,cur_width,self.height);
            let src = Rect::new((c as i32 - 32) * 32,0,cur_width,self.height);
            renderer.copy(&self.glyphs, Some(src), Some(dst));
            xpos += cur_width as i32;
        }
    }
}
