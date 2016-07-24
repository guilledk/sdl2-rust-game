use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::Renderer;

//Sprite

pub struct Sprite {

    pub img: Texture,
    pub rect: RectF,

}

impl Sprite {

    pub fn draw(&mut self, renderer: &mut Renderer, cam: &PointF) {
        let dest = Rect::new(
            (self.rect.pos.x - cam.x).round() as i32,
            (self.rect.pos.y - cam.y).round() as i32,
            self.rect.sze.x.round() as u32,
            self.rect.sze.y.round() as u32
        );
        renderer.copy(&self.img, None, Some(dest));
    }

}

//Point Float

#[derive(Debug)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub fn as_screen(&self, tilesize: f32) -> PointF {
        PointF {
            x: (self.x * tilesize),
            y: (self.y * tilesize)
        }
    }
    pub fn as_map(&self, tilesize: f32) -> PointF {
        PointF {
            x: (self.x / tilesize),
            y: (self.y / tilesize)
        }
    }
    pub fn floor(&self) -> PointF {
        PointF {
            x: self.x.floor(),
            y: self.y.floor()
        }
    }
    pub fn ceil(&self) -> PointF {
        PointF {
            x: self.x.ceil(),
            y: self.y.ceil()
        }
    }
}

//Rect Float

pub struct RectF {
    pub pos: PointF,
    pub sze: PointF,
}

impl RectF {
    pub fn to_sdl_rect(&self) -> Rect {
        Rect::new(
            self.pos.x.round() as i32,
            self.pos.y.round() as i32,
            self.sze.x.round() as u32,
            self.sze.y.round() as u32
        )
    }
}

pub fn lerp(a: &PointF, b: &PointF, f: f32) -> PointF {

    PointF {
        x: (a.x * (1.0 - f)) + (b.x * f),
        y: (a.y * (1.0 - f)) + (b.y * f)
    }

}
