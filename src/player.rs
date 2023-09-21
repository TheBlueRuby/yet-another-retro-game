use raylib::prelude::{KeyboardKey::*, RaylibDraw, Vector2};

use crate::types::Entity;

#[derive(Debug)]
pub struct Player {
    pub transform: Entity,
    pub texture: raylib::texture::Image,
}

impl Player {
    pub fn new(
        x: i32,
        y: i32,
        mv: Vector2,
        sp: Vector2,
        texture: raylib::texture::Image,
    ) -> Self {
        Self {
            transform: Entity::new(x, y, Vector2::new(0.0, 0.0), mv, sp),
            texture,
        }
    }

    pub fn handle_input(&mut self, rl: &raylib::RaylibHandle) {
        /* ---------------------------------- input --------------------------------- */
        if rl.is_key_down(KEY_W) && (self.transform.vel.y >= -self.transform.mv.y) {
            self.transform.vel.y -= self.transform.sp.y;
        }

        if rl.is_key_down(KEY_A) && (self.transform.vel.x >= -self.transform.mv.x) {
            self.transform.vel.x -= self.transform.sp.x;
        }

        if rl.is_key_down(KEY_S) && (self.transform.vel.y <= self.transform.mv.y) {
            self.transform.vel.y += self.transform.sp.y;
        }

        if rl.is_key_down(KEY_D) && (self.transform.vel.x <= self.transform.mv.x) {
            self.transform.vel.x += self.transform.sp.x;
        }
        
        /* -------------------------------- friction -------------------------------- */
        if self.transform.vel.x < 0.0 {
            self.transform.vel.x += 0.5;
        }
        if self.transform.vel.x > 0.0 {
            self.transform.vel.x -= 0.5;
        }

        if self.transform.vel.y < 0.0 {
            self.transform.vel.y += 0.5;
        }
        if self.transform.vel.y > 0.0 {
            self.transform.vel.y -= 0.5;
        }

        /* ----------------------------- updating player ---------------------------- */
        self.transform.x += self.transform.vel.x as i32;
        self.transform.y += self.transform.vel.y as i32;
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
