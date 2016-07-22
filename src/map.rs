extern crate sdl2;
extern crate image;
extern crate sdl2_image;

use sdl2::render::Texture;
use std::path::Path;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use utils::PointF;
use map::image::GenericImage;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Void,
    Grass,
    Dirt
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub ttype: TileType,
    pub metadata: u8,
    pub walkable: bool,
}

pub struct TileMap {
    pub tiles: [[Tile; 10]; 10],
    pub tilesize: u8,
    pub tileset: Texture,
}

impl TileMap {
    pub fn new(img_path: &Path, tset: Texture) -> TileMap {

        let mut map = TileMap {
            tiles: [[
                Tile {
                    ttype: TileType::Void,
                    metadata: 0,
                    walkable: false
                }; 10];
            10],
            tilesize: 64,
            tileset: tset
        };

        let img = image::open(img_path).unwrap();
        let (size_x, size_y) = img.dimensions();
        for y in 0..size_y {
            for x in 0..size_x {
                let pixel = img.get_pixel(x, y);

                let tile_type = match pixel[0] {
                    0 => TileType::Void,
                    1 => TileType::Grass,
                    2 => TileType::Dirt,
                    _ => TileType::Void
                };

                let meta = pixel[1];
                if meta > 8 { panic!("Wrong metadata @ x: {}, y: {}.", x, y) }
                let wable = if pixel[0] == 0 { false } else { true };

                map.tiles[x as usize][y as usize] = Tile {
                    ttype: tile_type,
                    metadata: meta,
                    walkable: wable
                };
            }
        }

        map
    }

    pub fn draw(&self, renderer: &mut Renderer, cam: &PointF) {
        for y in 0..10 {
            for x in 0..10 {
                let cur = self.tiles[x][y];

                if cur.ttype == TileType::Void { continue };

                let tmptsize: u32 = self.tilesize as u32;
                let src = Rect::new(cur.metadata as i32 * tmptsize as i32,0,tmptsize,tmptsize);
                let dst = Rect::new(
                    (x as i32 * tmptsize as i32) - cam.x as i32,
                    (y as i32 * tmptsize as i32) - cam.y as i32,
                    tmptsize,
                    tmptsize
                );

                renderer.copy(&self.tileset, Some(src), Some(dst));
            }
        }
    }

    pub fn inside(&self, point: PointF) -> bool {
        point.x >= 0.0 && point.x <= 9.0 && point.y >= 0.0 && point.y <= 9.0
    }

}
