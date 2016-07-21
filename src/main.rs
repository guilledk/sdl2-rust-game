extern crate time;
extern crate sdl2;
extern crate sdl2_image;

use time::PreciseTime;

mod game;
use game::{GameState, Player};

mod map;
use map::*;

mod utils;
use utils::*;

use std::path::Path;
use std::thread;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;

use sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};

fn main() {

    println!("Hello, world!");

    let ctx = sdl2::init().unwrap();
    sdl2_image::init(INIT_PNG | INIT_JPG).unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = video_ctx
    .window("SDL2 Rust 0.1.0", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

    let mut renderer = window
    .renderer()
    .build()
    .unwrap();

    let mut events = ctx.event_pump().unwrap();

    let mut gs = GameState {
        keys: [false, false, false, false, false],
        curmap: TileMap::new(
            &Path::new("res/test-mapfile.map"),
            &Path::new("res/default-tileset.png"),
            &renderer
        ),
        player: Player::new(
            PointF { x: 0.0, y: 0.0 },
            Sprite {
                img: renderer.load_texture(&Path::new("res/player.png")).unwrap(),
                rect: RectF {
                    pos: PointF { x: 0.0, y: 0.0 },
                    sze: PointF { x: 64.0, y: 64.0}
                }
            }
        ),
        camera: PointF { x: 0.0, y: 0.0 }
    };

    'event : loop {

        let start = PreciseTime::now();

        if gs.keys[0] { break 'event }
        for event in  events.poll_iter() {
            match event {
                Event::Quit{..}    => break 'event,
                Event::KeyDown{keycode: Some(keycode), ..} => keydown(keycode, &mut gs),
                Event::KeyUp{keycode: Some(keycode), ..}   => keyup(keycode, &mut gs),
                _                  => continue
            }
        }
        update(&mut gs);
        draw(&mut renderer, &mut gs);

        let end = PreciseTime::now();
        let delta = 16 - start.to(end).num_milliseconds();

        thread::sleep(std::time::Duration::from_millis(delta as u64));

    }

}

fn update(gs: &mut GameState) {
    //Vertical
    if gs.keys[1] {
        gs.player.move_up(&gs.curmap);
    }
    if gs.keys[3] {
        gs.player.move_down(&gs.curmap);
    }
    //Horizontal
    if gs.keys[2] {
        gs.player.move_left(&gs.curmap);
    }
    if gs.keys[4] {
        gs.player.move_right(&gs.curmap);
    }
    gs.player.update();

    let playerpos = gs.player.tilepos.as_screen(64.0);
    let newcampos = PointF {
            x: playerpos.x + 32.0 - 400.0,
            y: playerpos.y + 32.0 - 300.0
    };
    gs.camera = lerp(&gs.camera,&newcampos, 0.1);

}

fn draw(renderer: &mut Renderer, gs: &mut GameState) {

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.clear();

    gs.curmap.draw(renderer, &gs.camera);
    gs.player.draw(renderer, &gs.camera);

    renderer.present();

}

fn keydown(key: Keycode, gs: &mut GameState) {

    match key {

        Keycode::Escape => gs.keys[0] = true,
        Keycode::W      => gs.keys[1] = true,
        Keycode::A      => gs.keys[2] = true,
        Keycode::S      => gs.keys[3] = true,
        Keycode::D      => gs.keys[4] = true,
        _               => {},

    }

}
fn keyup(key: Keycode, gs: &mut GameState) {

    match key {

        Keycode::Escape => gs.keys[0] = false,
        Keycode::W      => gs.keys[1] = false,
        Keycode::A      => gs.keys[2] = false,
        Keycode::S      => gs.keys[3] = false,
        Keycode::D      => gs.keys[4] = false,
        _               => {},

    }

}
