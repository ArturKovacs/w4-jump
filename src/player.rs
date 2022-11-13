use crate::math::Vec2;
use crate::wasm4::*;

enum HookState {
    Ready,
    Pulling { pos: Vec2, max_dist: f32 },
    Hooked { pos: Vec2, max_dist: f32 },
}

pub struct Player {
    // CROSSHAIR
    crosshair_pos: Vec2,
    crosshair_vel: Vec2,

    hook_state: HookState,

    pos: Vec2,
    speed: Vec2,
    jump_was_released: bool,
}
impl Player {
    const SIZE: u32 = 10;

    pub const fn new() -> Self {
        Player {
            // Relative to the Player
            crosshair_pos: Vec2::new(30.0, -10.0),
            crosshair_vel: Vec2::zero(),

            hook_state: HookState::Ready,

            pos: Vec2::zero(),
            speed: Vec2::zero(),
            jump_was_released: true,
        }
    }

    pub fn update(&mut self) {
        const DECELERATION: f32 = 1.5;
        const HORIZ_ACCELERATION: f32 = 0.2;
        const MAX_H_SPEED: f32 = 8.0;
        const MAX_V_SPEED: f32 = 50.0;
        const FALL_ACC: f32 = 0.15;
        const JUMP_SPEED: f32 = -4.0;

        const CH_MAX_SPEED: f32 = 2.0;
        const CH_ACCELERATION: f32 = 0.1;

        const HOOK_PULL_SPEED: f32 = 2.0;

        // Horizontal
        if Button::Left.pressed(1) {
            self.crosshair_vel.x = self.crosshair_vel.x.min(0.0);
            self.crosshair_vel.x -= CH_ACCELERATION;
        } else if Button::Right.pressed(1) {
            self.crosshair_vel.x = self.crosshair_vel.x.max(0.0);
            self.crosshair_vel.x += CH_ACCELERATION;
        } else {
            self.crosshair_vel.x = 0.0;
        }

        // Vertical
        if Button::Up.pressed(1) {
            self.crosshair_vel.y = self.crosshair_vel.y.min(0.0);
            self.crosshair_vel.y -= CH_ACCELERATION;
        } else if Button::Down.pressed(1) {
            self.crosshair_vel.y = self.crosshair_vel.y.max(0.0);
            self.crosshair_vel.y += CH_ACCELERATION;
        } else {
            self.crosshair_vel.y = 0.0;
        }
        // let pressing_jump = Button::Btn1.pressed(1);
        // if self.jump_was_released && pressing_jump {
        //     self.jump_was_released = false;
        //     self.speed.y = JUMP_SPEED;
        // }
        // self.jump_was_released = !pressing_jump;

        let mut acceleration = Vec2::new(0.0, FALL_ACC);
        let mut apply_hook_constraint = |hook_pos: Vec2, max_dist| {
            let pos_to_hook = hook_pos - self.pos;
            if pos_to_hook.length() > max_dist {
                let pos_to_hook_norm = pos_to_hook.normalized();

                // First, correct the position so that the player is not further than the allowed
                // max distance
                self.pos = hook_pos - pos_to_hook_norm * max_dist;

                // Next, also correct the speed vector because it must not be at an angle more than
                // 90 degrees relative to the hook direction
                let speed_dot_hood_dir = self.speed.dot(pos_to_hook_norm);
                if speed_dot_hood_dir < 0.0 {
                    self.speed = self.speed - pos_to_hook_norm * speed_dot_hood_dir;
                }

                // apply a foce that's just enough to cancel out any force that drags
                // the player away from the hook
                // (directly setting the acceleration instead of specifying a force is
                // correct because our body has unit mass) F = m*a
                acceleration += pos_to_hook_norm * -(pos_to_hook_norm.dot(acceleration).min(0.0));
            }
        };

        match &mut self.hook_state {
            HookState::Ready => {
                if Button::Btn1.pressed(1) {
                    // Crosshair is relative to the player pos, but the hook target is absolute
                    self.hook_state = HookState::Pulling {
                        pos: self.crosshair_pos + self.pos,
                        max_dist: self.crosshair_pos.length(),
                    };
                }
            }
            HookState::Pulling {
                pos: hook_pos,
                max_dist,
            } => {
                // self.pos = self.pos.move_to(hook_pos, HOOK_PULL_SPEED);
                *max_dist = (*max_dist - HOOK_PULL_SPEED).max(4.0);
                apply_hook_constraint(*hook_pos, *max_dist);
                if !Button::Btn1.pressed(1) {
                    self.hook_state = HookState::Hooked {
                        pos: *hook_pos,
                        max_dist: (self.pos - *hook_pos).length(),
                    };
                }
            }
            HookState::Hooked {
                pos: hook_pos,
                max_dist,
            } => {
                // TODO: maybe this should be split up to multiple steps because
                // quick non-linear motion can happen when hooked, and big discrete
                // steps may approximate the motion very badly

                apply_hook_constraint(*hook_pos, *max_dist);

                // // now apply the hook distance constraint
                // let hook_to_pos = self.pos - *hook_pos;
                // if hook_to_pos.length() > *max_dist {
                //     let hook_to_pos_norm = hook_to_pos.normalized();

                //     // apply a foce that's just enough to cancel out any force that drags
                //     // the player away from the hook
                //     // (directly setting the acceleration instead of specifying a force is
                //     // correct because our body has unit mass) F = m*a
                //     acceleration += hook_to_pos_norm * -(hook_to_pos_norm.dot(acceleration).max(0.0));

                //     // self.pos = hook_pos + hook_to_pos_norm * max_dist;
                // }

                if Button::Btn2.pressed(1) {
                    self.hook_state = HookState::Ready;
                }
            }
        }
        self.crosshair_vel.x = self.crosshair_vel.x.max(-CH_MAX_SPEED).min(CH_MAX_SPEED);
        self.crosshair_vel.y = self.crosshair_vel.y.max(-CH_MAX_SPEED).min(CH_MAX_SPEED);
        self.crosshair_pos += self.crosshair_vel;

        self.speed += acceleration;
        self.speed.x = self.speed.x.max(-MAX_H_SPEED).min(MAX_H_SPEED);
        self.speed.y = self.speed.y.max(-MAX_V_SPEED).min(MAX_V_SPEED);
        self.pos += self.speed;

        // Check ground collision
        if self.get_bottom_center().y > 0.0 {
            self.pos.y = -(Self::SIZE as f32 / 2.0);
            self.speed.y = 0.0;
        }
    }

    pub fn draw(&self, pos_offset: Vec2) {
        let offset_player_pos = self.pos + pos_offset;
        unsafe { *DRAW_COLORS = 0x04 }
        rect(
            offset_player_pos.x as i32 - Self::SIZE as i32 / 2,
            offset_player_pos.y as i32 - Self::SIZE as i32 / 2,
            Self::SIZE,
            Self::SIZE,
        );

        // CROSSHAIR
        const CROSSHAIR_SIZE: i32 = 4;
        let offset_ch_pos = offset_player_pos + self.crosshair_pos;
        let chx = offset_ch_pos.x as i32;
        let chy = offset_ch_pos.y as i32;
        line(chx, chy - CROSSHAIR_SIZE, chx, chy + CROSSHAIR_SIZE);
        line(chx - CROSSHAIR_SIZE, chy, chx + CROSSHAIR_SIZE, chy);

        // HOOK
        match self.hook_state {
            HookState::Ready => {
                // Nothing to draw
            }
            HookState::Pulling { pos: hook_pos, .. } | HookState::Hooked { pos: hook_pos, .. } => {
                let offset_hook_pos = hook_pos + pos_offset;
                line(
                    offset_hook_pos.x as i32,
                    offset_hook_pos.y as i32,
                    offset_player_pos.x as i32,
                    offset_player_pos.y as i32,
                )
            }
        }
    }

    pub fn get_center(&self) -> Vec2 {
        self.pos
    }

    pub fn get_bottom_center(&self) -> Vec2 {
        const HALF_SIZE: f32 = Player::SIZE as f32 / 2.0;
        self.pos
            + Vec2 {
                x: 0.0,
                y: HALF_SIZE,
            }
    }
}
