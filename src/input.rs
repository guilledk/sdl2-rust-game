use sdl2::keyboard::Keycode;
use utils::PointF;
use sdl2::mouse::{MouseState, MouseUtil};

pub struct Mouse {
    pub screenpos: PointF,
    pub tilepos: PointF,
    pub state: MouseState,
}

impl Mouse {
    pub fn new(util: &MouseUtil) -> Mouse {
        let (nstate, mx, my) = util.mouse_state();
        Mouse {
            screenpos: PointF { x:mx as f32, y:my as f32 },
            tilepos: PointF { x: 0.0, y: 0.0 },
            state: nstate
        }
    }
    pub fn mouse_update(&mut self, util: &MouseUtil, cam: &PointF){
        let (nstate, mx, my) = util.mouse_state();
        self.state = nstate;
        self.screenpos.x = mx as f32;
        self.screenpos.y = my as f32;
        self.tilepos = PointF{ x: self.screenpos.x + cam.x, y: self.screenpos.y + cam.y }.as_map(64.0);
    }
}

/*
 *keys:
 *  0: Escape
 *  1: w
 *  2: a
 *  3: s
 *  4: d
 *  5: F1
 *  6: q
 *  7: e
 */

const NUM_KEYS: usize = 9;

pub struct Keyboard {
    pressed: [bool; NUM_KEYS],
    released: [bool; NUM_KEYS],
    down: [bool; NUM_KEYS],
}

impl Keyboard {

    pub fn new() -> Keyboard {
        Keyboard {
            pressed: [false; NUM_KEYS],
            released: [false; NUM_KEYS],
            down: [false; NUM_KEYS]
        }
    }

    pub fn begin_frame(&mut self) {
        self.pressed = [false; NUM_KEYS];
        self.released = [false; NUM_KEYS];
    }

    pub fn keydown(&mut self, key: Keycode) {
        match key {

            Keycode::Escape => { self.down[0] = true; self.pressed[0] = true },
            Keycode::W      => { self.down[1] = true; self.pressed[1] = true },
            Keycode::A      => { self.down[2] = true; self.pressed[2] = true },
            Keycode::S      => { self.down[3] = true; self.pressed[3] = true },
            Keycode::D      => { self.down[4] = true; self.pressed[4] = true },
            Keycode::F1     => { self.down[5] = true; self.pressed[5] = true },
            Keycode::Space  => { self.down[6] = true; self.pressed[6] = true },
            Keycode::Q      => { self.down[7] = true; self.pressed[7] = true },
            Keycode::E      => { self.down[8] = true; self.pressed[8] = true },
            _               => {},

        }
    }

    pub fn keyup(&mut self, key: Keycode) {
        match key {

            Keycode::Escape => { self.down[0] = false; self.released[0] = true },
            Keycode::W      => { self.down[1] = false; self.released[1] = true },
            Keycode::A      => { self.down[2] = false; self.released[2] = true },
            Keycode::S      => { self.down[3] = false; self.released[3] = true },
            Keycode::D      => { self.down[4] = false; self.released[4] = true },
            Keycode::F1     => { self.down[5] = false; self.released[5] = true },
            Keycode::Space  => { self.down[6] = false; self.released[6] = true },
            Keycode::Q      => { self.down[7] = false; self.released[7] = true },
            Keycode::E      => { self.down[8] = false; self.released[8] = true },
            _               => {},

        }
    }

    pub fn is_down(&self, key: Keycode) -> bool {
        match key {

            Keycode::Escape => self.down[0],
            Keycode::W      => self.down[1],
            Keycode::A      => self.down[2],
            Keycode::S      => self.down[3],
            Keycode::D      => self.down[4],
            Keycode::F1     => self.down[5],
            Keycode::Space  => self.down[6],
            Keycode::Q      => self.down[7],
            Keycode::E      => self.down[8],
            _               => panic!("Keycode: {:?} not implemented.", key),

        }
    }

    pub fn was_pressed(&self, key: Keycode) -> bool {
        match key {

            Keycode::Escape => self.pressed[0],
            Keycode::W      => self.pressed[1],
            Keycode::A      => self.pressed[2],
            Keycode::S      => self.pressed[3],
            Keycode::D      => self.pressed[4],
            Keycode::F1     => self.pressed[5],
            Keycode::Space  => self.pressed[6],
            Keycode::Q      => self.pressed[7],
            Keycode::E      => self.pressed[8],
            _               => panic!("Keycode: {:?} not implemented.", key),

        }
    }

    pub fn was_released(&self, key: Keycode) -> bool {
        match key {

            Keycode::Escape => self.released[0],
            Keycode::W      => self.released[1],
            Keycode::A      => self.released[2],
            Keycode::S      => self.released[3],
            Keycode::D      => self.released[4],
            Keycode::F1     => self.released[5],
            Keycode::Space  => self.released[6],
            Keycode::Q      => self.released[7],
            Keycode::E      => self.released[8],
            _               => panic!("Keycode: {:?} not implemented.", key),

        }
    }

}
