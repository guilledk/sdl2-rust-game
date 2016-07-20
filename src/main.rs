extern crate time;
extern crate sdl2;
extern crate sdl2_image;

use time::{PreciseTime};

mod game;
use game::GameState;

mod map;
use map::*;

use std::path::Path;
use std::thread;

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

    renderer.set_scale(0.5, 0.5);

    let mut events = ctx.event_pump().unwrap();
    let texture = renderer.load_texture(&Path::new("res/rust-logo.png")).unwrap();

    let mut gs = GameState {
        keys: [false, false, false, false, false],
        curmap: TileMap::new(
            &Path::new("res/test-mapfile.map"),
            &Path::new("res/default-tileset.png"),
            &renderer
        )
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
        draw(&mut renderer, &gs);

        let end = PreciseTime::now();
        let delta = 16 - start.to(end).num_milliseconds();

        thread::sleep(std::time::Duration::from_millis(delta as u64));

    }

}

fn update(gs: &mut GameState) {



}

fn draw(renderer: &mut Renderer, gs: &GameState) {

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.clear();

    gs.curmap.draw(renderer);

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
