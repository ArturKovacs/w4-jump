#[cfg(feature = "buddy-alloc")]
mod alloc;
mod math;
mod wasm4;
use core::{
    cell::RefCell,
    ops::{Deref},
};

use math::Vec2;
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

struct Player {
    pos: Vec2,
    speed: Vec2,

    jump_was_released: bool,
}
impl Player {
    const SIZE: u32 = 10;

    fn update(&mut self) {
        const DECELERATION: f32 = 1.5;
        const HORIZ_ACCELERATION: f32 = 0.2;
        const MAX_H_SPEED: f32 = 4.0;
        const MAX_V_SPEED: f32 = 50.0;
        const FALL_SPEED: f32 = 0.15;
        const JUMP_SPEED: f32 = -4.0;

        // Horizontal
        if btn_pressed(1, GAMEPAD_LEFT) {
            self.speed.x -= HORIZ_ACCELERATION;
        } else if btn_pressed(1, GAMEPAD_RIGHT) {
            self.speed.x += HORIZ_ACCELERATION;
        } else {
            self.speed.x /= DECELERATION;
        }
        self.speed.x = self.speed.x.max(-MAX_H_SPEED).min(MAX_H_SPEED);
        // Vertical
        let pressing_jump = btn_pressed(1, GAMEPAD_1);
        if self.jump_was_released && pressing_jump {
            self.jump_was_released = false;
            self.speed.y = JUMP_SPEED;
        }
        self.jump_was_released = !pressing_jump;
        // if btn_pressed(1, GAMEPAD_UP) {
        //     self.speed.y -= 0.1;
        // } else if btn_pressed(1, GAMEPAD_DOWN) {
        //     self.speed.y += 0.1;
        // } else {
        //     self.speed.y /= DECELERATION;
        // }

        self.speed.y += FALL_SPEED;
        self.speed.y = self.speed.y.max(-MAX_V_SPEED).min(MAX_V_SPEED);

        self.pos += self.speed;
        if self.pos.y > 0.0 {
            self.pos.y = 0.0;
            self.speed.y = 0.0;
        }
    }

    fn draw(&self, pos_offset: Vec2) {
        let target_pos = self.pos + pos_offset;
        unsafe { *DRAW_COLORS = 0x04 }
        rect(
            target_pos.x as i32,
            target_pos.y as i32 - Self::SIZE as i32,
            Self::SIZE,
            Self::SIZE,
        );
    }
}

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
        let to_player = self.player.pos - self.cam_pos;
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
        rect(
            0,
            pos_offset.y as i32,
            160,
            ground_size,
        );
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
    player: Player {
        pos: Vec2::new(0.0, 0.0),
        speed: Vec2::new(0.0, 0.0),
        jump_was_released: true,
    },
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
