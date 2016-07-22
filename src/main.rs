extern crate time;
extern crate sdl2;
extern crate sdl2_image;

use time::PreciseTime;

mod game;
use game::*;

mod map;
use map::*;

mod input;
use input::*;
use sdl2::keyboard::Keycode;

mod utils;
use utils::*;

use std::path::Path;
use std::thread;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::render::Renderer;

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
        gamemode: GameMode::Quest,
        keys: Keyboard::new(),
        mouse: Mouse::new(&ctx.mouse()),
        curmap: TileMap::new(
            &Path::new("res/map001.png"),
            renderer.load_texture(&Path::new("res/default-tileset.png")).unwrap()
        ),
        player: Player::new(
            PointF { x: 0.0, y: 0.0 },
            Sprite {
                img: renderer.load_texture(&Path::new("res/player.png")).unwrap(),
                rect: RectF {
                    pos: PointF { x: 0.0, y: 0.0 },
                    sze: PointF { x: 64.0, y: 64.0 }
                }
            }
        ),
        camera: PointF { x: 0.0, y: 0.0 }
    };

    'event : loop {

        let start = PreciseTime::now();

        gs.keys.begin_frame();
        gs.mouse.mouse_update(&ctx.mouse(), &gs.camera);

        for event in  events.poll_iter() {
            match event {
                Event::Quit{..}    => break 'event,
                Event::KeyDown{keycode: Some(keycode), ..} => gs.keys.keydown(keycode),
                Event::KeyUp  {keycode: Some(keycode), ..} => gs.keys.keyup(keycode),
                _                  => continue
            }
        }

        if gs.keys.was_pressed(Keycode::Escape) { break 'event }

        update(&mut gs);
        draw(&mut renderer, &mut gs);

        let end = PreciseTime::now();
        let delta = 16 - start.to(end).num_milliseconds();

        thread::sleep(std::time::Duration::from_millis(delta as u64));

    }

}

fn update(gs: &mut GameState) {

    if gs.gamemode == GameMode::Quest {

        if gs.keys.was_pressed(Keycode::F1) {
            gs.gamemode = GameMode::Developer;
        }
        //Vertical
        if gs.keys.is_down(Keycode::W) {
            gs.player.move_up(&gs.curmap);
        }
        if gs.keys.is_down(Keycode::S) {
            gs.player.move_down(&gs.curmap);
        }
        //Horizontal
        if gs.keys.is_down(Keycode::A) {
            gs.player.move_left(&gs.curmap);
        }
        if gs.keys.is_down(Keycode::D) {
            gs.player.move_right(&gs.curmap);
        }

        gs.player.update();

        let playerpos = gs.player.tilepos.as_screen(64.0);
        let newcampos = PointF {
                x: playerpos.x + 32.0 - 400.0,
                y: playerpos.y + 32.0 - 300.0
        };
        gs.camera = lerp(&gs.camera,&newcampos, 0.05);

    } else {

        if gs.keys.was_pressed(Keycode::F1) {
            gs.gamemode = GameMode::Quest;
        }

        const DEBUG_CAM_SPEED: f32 = 16.0;

        let speeder = if gs.keys.is_down(Keycode::Space) { 4.0 } else { 1.0 };

        //Vertical
        if gs.keys.is_down(Keycode::W) {
            gs.camera.y -= DEBUG_CAM_SPEED * speeder;
        }
        if gs.keys.is_down(Keycode::S) {
            gs.camera.y += DEBUG_CAM_SPEED * speeder;
        }
        //Horizontal
        if gs.keys.is_down(Keycode::A) {
            gs.camera.x -= DEBUG_CAM_SPEED * speeder;
        }
        if gs.keys.is_down(Keycode::D) {
            gs.camera.x += DEBUG_CAM_SPEED * speeder;
        }

    }

}

fn draw(renderer: &mut Renderer, gs: &mut GameState) {

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.clear();

    gs.curmap.draw(renderer, &gs.camera);
    gs.player.draw(renderer, &gs.camera);

    if gs.gamemode == GameMode::Developer {

        let tilehover = PointF { x: gs.mouse.tilepos.x.floor(), y: gs.mouse.tilepos.y.floor() }.as_screen(64.0);
        let hover_rect = Rect::new(tilehover.x as i32 - gs.camera.x as i32, tilehover.y as i32 - gs.camera.y as i32, 64, 64);
        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        renderer.draw_rect(hover_rect);

    }

    renderer.present();

}
