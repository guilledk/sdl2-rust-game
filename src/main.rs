extern crate time;
extern crate sdl2;
extern crate sdl2_image;

use time::{Duration, PreciseTime};

mod gamestate;
use gamestate::GameState;
use gamestate::Sprite;
use gamestate::RectF;

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

    let mut events = ctx.event_pump().unwrap();
    let texture = renderer.load_texture(&Path::new("res/rust-logo.png")).unwrap();

    let mut gs = GameState {
        logo: Sprite { img: &texture, rect: RectF { x: -512.0, y: 300.0 - 256.0, w: 512.0, h: 512.0 } },
        keys: [false, false, false, false, false]
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

        let delta: Duration = start.to(PreciseTime::now());

        thread::sleep_ms((Duration::microseconds(16666) - delta).num_milliseconds() as u32);

    }

}

fn update(gs: &mut GameState) {

    gs.logo.rect.x += 1000.0 / 60.0;
    if gs.logo.rect.x > 800.0 {
        gs.logo.rect.x = -512.0;
    }

}

fn draw(renderer: &mut Renderer, gs: &GameState) {

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.clear();
    gs.logo.draw(renderer);

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
