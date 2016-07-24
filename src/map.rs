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
    pub tiles: Vec<Vec<Tile>>,
    pub tilesize: u8,
    pub tileset: Texture,
    pub sze_x: u32,
    pub sze_y: u32,
}

impl TileMap {
    pub fn new(img_path: &Path, tset: Texture) -> TileMap {

        let img = image::open(img_path).unwrap();
        let (size_x, size_y) = img.dimensions();

        let mut map = TileMap {
            tiles: Vec::new(),
            tilesize: 64,
            tileset: tset,
            sze_x: size_x,
            sze_y: size_y
        };
        for x in 0..size_x {
            map.tiles.push(Vec::new());
            for y in 0..size_y {
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

                map.tiles[x as usize].push(
                    Tile {
                        ttype: tile_type,
                        metadata: meta,
                        walkable: wable
                    }
                );
            }
        }

        map
    }

    pub fn add_row(&mut self) {
        for x in 0..self.sze_x {
            self.tiles[x as usize].push(
                Tile {
                    ttype: TileType::Grass,
                    metadata: 0,
                    walkable: true
                }
            );
        }
        self.sze_y += 1;
    }

    pub fn add_column(&mut self) {
        self.tiles.push(Vec::new());

        self.sze_x += 1;
        for y in 0..self.sze_y {
            self.tiles[(self.sze_x - 1) as usize].push(
                Tile {
                    ttype: TileType::Grass,
                    metadata: 0,
                    walkable: true
                }
            );
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, cam: &PointF) {

        let min = cam.as_map(64.0).floor();
        let (r_x,r_y) =  renderer.output_size().unwrap();
        let max = PointF { x: r_x as f32 + cam.x, y: r_y as f32 + cam.y }.as_map(64.0).ceil();

        for y in min.y as i32..max.y as i32 {
            for x in min.x as i32..max.x as i32 {

                if x < 0 || x > self.sze_x as i32 - 1 || y < 0 || y > self.sze_y as i32 - 1 { continue };

                let cur = self.tiles[x as usize][y as usize];

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
        point.x >= 0.0 && point.x <= (self.sze_x - 1) as f32 && point.y >= 0.0 && point.y <= (self.sze_y - 1) as f32
    }

}
