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
        mv_x: f32,
        mv_y: f32,
        sp_x: f32,
        sp_y: f32,
        texture: raylib::texture::Image,
    ) -> Self {
        Self {
            transform: Entity::new(x, y, 0.0, 0.0, mv_x, mv_y, sp_x, sp_y),
            texture,
        }
    }

    pub fn handle_input(&mut self, rl: &raylib::RaylibHandle) {
        /* ---------------------------------- input --------------------------------- */
        if rl.is_key_down(KEY_W) && (self.transform.vel_y >= -self.transform.mv_y) {
            self.transform.vel_y -= self.transform.sp_y;
        }

        if rl.is_key_down(KEY_A) && (self.transform.vel_x >= -self.transform.mv_x) {
            self.transform.vel_x -= self.transform.sp_x;
        }

        if rl.is_key_down(KEY_S) && (self.transform.vel_y <= self.transform.mv_y) {
            self.transform.vel_y += self.transform.sp_y;
        }

        if rl.is_key_down(KEY_D) && (self.transform.vel_x <= self.transform.mv_x) {
            self.transform.vel_x += self.transform.sp_x;
        }
        
        /* -------------------------------- friction -------------------------------- */
        if self.transform.vel_x < 0.0 {
            self.transform.vel_x += 0.5;
        }
        if self.transform.vel_x > 0.0 {
            self.transform.vel_x -= 0.5;
        }

        if self.transform.vel_y < 0.0 {
            self.transform.vel_y += 0.5;
        }
        if self.transform.vel_y > 0.0 {
            self.transform.vel_y -= 0.5;
        }

        /* ----------------------------- updating player ---------------------------- */
        self.transform.x += self.transform.vel_x as i32;
        self.transform.y += self.transform.vel_y as i32;
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
