use raylib::prelude::{KeyboardKey::*, RaylibDraw};

use crate::types::Entity;

#[derive(Debug)]
pub struct Player {
    pub(crate) transform: Entity,
    pub(crate) texture: raylib::texture::Image,
}

impl Player {
    pub fn new(
        x: i32,
        y: i32,
        mv_x: i32,
        mv_y: i32,
        sp_x: i32,
        sp_y: i32,
        texture: raylib::texture::Image,
    ) -> Self {
        Self {
            transform: Entity::new(x, y, 0, 0, mv_x, mv_y, sp_x, sp_y),
            texture,
        }
    }

    pub fn handle_input(&mut self, rl: &raylib::RaylibHandle) {
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

    pub fn draw(
        &self,
        t: &mut raylib::prelude::RaylibTextureMode<'_, raylib::prelude::RaylibDrawHandle<'_>>,
        player_tex: &raylib::texture::Texture2D,
    ) {
        t.draw_texture_ex(
            player_tex,
            raylib::prelude::Vector2::new(self.transform.x as f32, self.transform.y as f32),
            0.0,
            1.0,
            raylib::prelude::Color::WHITE,
        )
    }
}
