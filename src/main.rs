extern crate sdl2;
extern crate sdl2_image;

mod gamestate;
use gamestate::GameState;

use std::path::Path;
use sdl2::event::Event;
use sdl2::rect::Rect;
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

    let dst = Rect::new(400 - 256, 300 - 256, 512, 512);
    let texture = renderer.load_texture(&Path::new("res/rust-logo.png")).unwrap();

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.copy(&texture, None, Some(dst));

    renderer.present();

    let mut events = ctx.event_pump().unwrap();

    let mut gs = GameState { keys: [false, false, false, false, false] };

    'event : loop {
        if gs.keys[0] { break 'event }
        for event in  events.poll_iter() {
            match event {
                Event::Quit{..}    => break 'event,
                Event::KeyDown{keycode: Some(keycode), ..} => { keydown(keycode, &mut gs); println!("keys: {:?}", gs.keys) },
                Event::KeyUp{keycode: Some(keycode), ..}   => { keyup(keycode, &mut gs); println!("keys: {:?}", gs.keys) },
                _                  => continue
            }
        }
    }

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
