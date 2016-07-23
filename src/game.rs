use map::TileMap;
use sdl2::render::Renderer;
use utils::*;
use text::GlyphAtlas;
use input::{Keyboard, Mouse};

#[derive (PartialEq)]
pub enum GameMode {
    Quest,
    Developer
}

pub struct GameState {
    pub gamemode: GameMode,
    pub keys: Keyboard,
    pub mouse: Mouse,
    pub curmap: TileMap,
    pub player: Player,
    pub camera: PointF,
    pub default_atlas: GlyphAtlas,
}

 pub struct Player {
     pub tilepos: PointF,
     pub sprite: Sprite,
     moving: bool,
     target: PointF,
 }

impl Player {

    pub fn new(tpos: PointF, sprt: Sprite) -> Player {
        Player {
            tilepos: tpos,
            sprite: sprt,
            moving: false,
            target: PointF { x: 0.0, y: 0.0 }
        }
    }

    pub fn update(&mut self) {
        self.tilepos = lerp(&self.tilepos, &self.target, 0.1);
        self.sprite.rect.pos = self.tilepos.as_screen(64.0);
        if self.moving && (self.tilepos.x - self.target.x).abs() < 0.1 && (self.tilepos.y - self.target.y).abs() < 0.1 {
            self.moving = false;
        }
    }
    pub fn draw(&mut self, renderer: &mut Renderer, cam: &PointF) {
        self.sprite.draw(renderer, cam);
    }

    pub fn move_down(&mut self, map: &TileMap) {
        let inside_map = map.inside(PointF { x: self.target.x, y: self.target.y + 1.0 });
        let can_walk = inside_map && map.tiles[self.target.x as usize][self.target.y as usize + 1].walkable;
        if !self.moving && can_walk {
            self.moving = true;
            self.target.y = self.target.y + 1.0;
        }
    }
    pub fn move_up(&mut self, map: &TileMap) {
        let inside_map = map.inside(PointF { x: self.target.x, y: self.target.y - 1.0 });
        let can_walk = inside_map && map.tiles[self.target.x as usize][self.target.y as usize - 1].walkable;
        if !self.moving && can_walk {
            self.moving = true;
            self.target.y = self.target.y - 1.0;
        }
    }
    pub fn move_right(&mut self, map: &TileMap) {
        let inside_map = map.inside(PointF { x: self.target.x + 1.0, y: self.target.y });
        let can_walk = inside_map && map.tiles[self.target.x as usize + 1][self.target.y as usize].walkable;
        if !self.moving && can_walk {
            self.moving = true;
            self.target.x = self.target.x + 1.0;
        }
    }
    pub fn move_left(&mut self, map: &TileMap) {
        let inside_map = map.inside(PointF { x: self.target.x - 1.0, y: self.target.y });
        let can_walk = inside_map && map.tiles[self.target.x as usize - 1][self.target.y as usize].walkable;
        if !self.moving && can_walk {
            self.moving = true;
            self.target.x = self.target.x - 1.0;
        }
    }

}
