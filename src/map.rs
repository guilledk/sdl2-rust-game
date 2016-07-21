extern crate sdl2;
extern crate sdl2_image;

use sdl2::render::Texture;
use sdl2_image::LoadTexture;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use std::io::BufReader;
use utils::PointF;

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

    pub fn new(map_path: &Path, tileset_path: &Path, renderer: &Renderer) -> TileMap{

        let raw_map_file = File::open(&map_path).unwrap();
        let map_reader = BufReader::new(raw_map_file);

        let mut map = TileMap {
            tiles: [[
                Tile {
                    ttype: TileType::Void,
                    metadata: 0,
                    walkable: false
                }; 10];
            10],
            tilesize: 64,
            tileset: renderer.load_texture(tileset_path).unwrap()
        };

        let mut y: usize = 0;
        for line in map_reader.lines() {
            let mut x: usize = 0;
            for chr in line.unwrap().split(",") {
                match chr {
                    //Full
                    "0" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 0, walkable: true },

                    //Corner
                    "1" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 1, walkable: true },
                    "6" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 6, walkable: true },
                    "7" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 7, walkable: true },
                    "8" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 8, walkable: true },

                    "2" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 2, walkable: true },
                    "3" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 3, walkable: true },
                    "4" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 4, walkable: true },
                    "5" => map.tiles[x][y] = Tile { ttype: TileType::Grass, metadata: 5, walkable: true },
                     _  => map.tiles[x][y] = Tile { ttype: TileType::Void,  metadata: 0, walkable: false }
                }
                x += 1;
            }
            y += 1;
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
