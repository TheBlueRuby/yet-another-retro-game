use raylib::prelude::{Color, KeyboardKey::*, RaylibDraw, Vector2};
use tiled::Map;

use crate::{
    level::check_tile_at,
    types::{Hitbox, Transform},
};

#[derive(Debug)]
pub struct Player {
    pub transform: Transform,
    pub texture: raylib::texture::Image,
    pub grounded: bool,
}

impl Player {
    pub fn new(
        x: i32,
        y: i32,
        mv: Vector2,
        sp: Vector2,
        hitbox: Hitbox,
        texture: raylib::texture::Image,
    ) -> Self {
        Self {
            transform: Transform::new(x, y, Vector2::new(0.0, 0.0), mv, sp, hitbox),
            texture,
            grounded: false,
        }
    }

    pub fn handle_input(&mut self, rl: &raylib::RaylibHandle) {
        /* ---------------------------------- input --------------------------------- */
        if self.grounded
            && ((rl.is_key_down(KEY_W) || rl.is_key_down(KEY_SPACE))
                && (self.transform.vel.y >= -self.transform.max_vel.y))
        {
            self.transform.vel.y -= 12.0;
        }

        if rl.is_key_down(KEY_A) && (self.transform.vel.x >= -self.transform.max_vel.x) {
            self.transform.vel.x -= self.transform.mvmt_spd.x;
        }

        if !self.grounded
            && (rl.is_key_down(KEY_S) && (self.transform.vel.y <= self.transform.max_vel.y))
        {
            self.transform.vel.y += self.transform.mvmt_spd.y;
        }

        if rl.is_key_down(KEY_D) && (self.transform.vel.x <= self.transform.max_vel.x) {
            self.transform.vel.x += self.transform.mvmt_spd.x;
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
        //gravity
        if !self.grounded {
            self.transform.vel.y += 0.75;
        } else if self.transform.vel.y >= 0.0 {
            self.transform.vel.y = 0.0;
        }

        self.transform.x += self.transform.vel.x as i32;
        self.transform.y += self.transform.vel.y as i32;

        if self.transform.x < 0 {
            self.transform.x = 0;
        }
        if self.transform.y < 0 {
            self.transform.y = 0;
        }
        if self.transform.x >= 310 {
            self.transform.x = 310;
        }
        if self.transform.y >= 170 {
            self.transform.y = 170;
        }

        self.transform.hitbox.rect.x = self.transform.x as f32 + self.transform.hitbox.offset.x;
        self.transform.hitbox.rect.y = self.transform.y as f32 + self.transform.hitbox.offset.y;
    }

    pub fn check_grounded(&mut self, map: &Map) {
        let sh = self.transform.hitbox.rect;
        let tile_id = check_tile_at(
            map,
            ((sh.x + sh.width / 2.0) / 16.0) as i32,
            ((sh.y + sh.height) / 16.0) as i32,
        );
        
        //15 is blank
        self.grounded = tile_id != 15;
    }

    pub fn draw(
        &self,
        t: &mut raylib::prelude::RaylibTextureMode<'_, raylib::prelude::RaylibDrawHandle<'_>>,
        player_tex: &raylib::texture::Texture2D,
        is_debug: bool,
    ) {
        t.draw_texture_ex(
            player_tex,
            raylib::prelude::Vector2::new(self.transform.x as f32, self.transform.y as f32),
            0.0,
            1.0,
            Color::WHITE,
        );
        if is_debug {
            t.draw_rectangle_lines_ex(self.transform.hitbox.rect, 1, Color::RED);
        }
    }
}
