use raylib::prelude::KeyboardKey::*;

use crate::types::Entity;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub(crate) transform: Entity,
}

impl Player {
    pub fn new(x: i32, y: i32, mv_x: i32, mv_y: i32, sp_x: i32, sp_y: i32) -> Self {
        Self {
            transform: Entity::new(x, y, 0, 0, mv_x, mv_y, sp_x, sp_y),
        }
    }

    pub fn handle_input(&mut self, rl: &mut raylib::RaylibHandle) {
        if rl.is_key_down(KEY_W) && self.transform.vel_y >= -self.transform.mv_y {
            self.transform.vel_y -= self.transform.sp_y;
        }

        if rl.is_key_down(KEY_A) && self.transform.vel_x >= -self.transform.mv_x {
            self.transform.vel_x -= self.transform.sp_x;
        }

        if rl.is_key_down(KEY_S) && self.transform.vel_y <= self.transform.mv_y {
            self.transform.vel_y += self.transform.sp_y;
        }

        if rl.is_key_down(KEY_D) && self.transform.vel_x <= self.transform.mv_x {
            self.transform.vel_x += self.transform.sp_x;
        }

        if self.transform.vel_x < 0 {
            self.transform.vel_x += 2;
        }
        if self.transform.vel_x > 0 {
            self.transform.vel_x -= 2;
        }

        if self.transform.vel_y < 0 {
            self.transform.vel_y += 2;
        }
        if self.transform.vel_y > 0 {
            self.transform.vel_y -= 2;
        }

        self.transform.x += self.transform.vel_x;
        self.transform.y += self.transform.vel_y;
    }
}
