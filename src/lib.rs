#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod math;
mod player;

use core::{cell::RefCell, ops::Deref};

use math::Vec2;
use player::Player;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

fn noise2d(x: f32, y: f32) -> f32 {
    let v = Vec2::new(x, y);
    let c = Vec2::new(12.9898, 78.233);
    (v.dot(c).sin() * 43758.5453).fract()
}

struct World {
    player: Player,
    cam_pos: Vec2,
    frame_cnt: u32,
}
impl World {
    fn update(&mut self) {
        text(format!("Frame: {}", self.frame_cnt), 10, 10);
        self.frame_cnt += 1;

        self.player.update();
        // Move the camera towards the player
        let to_player = self.player.get_center() - self.cam_pos;
        self.cam_pos += to_player * 0.1;

        self.draw();
    }

    fn draw(&self) {
        self.draw_bg_particles();
        let pos_offset = self.cam_pos * -1.0 + Vec2::from(SCREEN_SIZE / 2);
        self.player.draw(pos_offset);

        // Draw ground
        unsafe { *DRAW_COLORS = 0x02 }
        let ground_size = 200;
        rect(0, pos_offset.y as i32, 160, ground_size);
    }

    fn draw_bg_particles(&self) {
        let cx = self.cam_pos.x as i32;
        let cy = self.cam_pos.y as i32;

        set_pixels(|x, y, c| {
            let has_particle = noise2d((x + cx) as f32, (y + cy) as f32) > 0.99;
            if has_particle {
                2
            } else {
                c
            }
        });
    }
}

struct WordPtr(RefCell<World>);
unsafe impl Sync for WordPtr {}
impl Deref for WordPtr {
    type Target = RefCell<World>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
static WORLD: WordPtr = WordPtr(RefCell::new(World {
    cam_pos: Vec2::new(0.0, 0.0),
    player: Player::new(),
    frame_cnt: 0,
}));

#[no_mangle]
fn start() {
    trace("starting");
    unsafe {
        *PALETTE = [0xc4f0c2, 0x1e606e, 0x5ab9a8, 0xd17c7c];
    }
}

#[no_mangle]
fn update() {
    WORLD.borrow_mut().update();

    // unsafe { *DRAW_COLORS = 2 }
    // text("Hello from Rust!", 10, 10);

    // let gamepad = unsafe { *GAMEPAD1 };
    // if gamepad & BUTTON_1 != 0 {
    //     unsafe { *DRAW_COLORS = 4 }
    // }
    // blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    // text("Press X to blink", 16, 90);
}
