use raylib::prelude::{Color, KeyboardKey::*, RaylibDraw, Texture2D, Vector2};
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
        max_vel: Vector2,
        movement_speed: Vector2,
        hitbox: Hitbox,
        texture: raylib::texture::Image,
    ) -> Self {
        Self {
            transform: Transform::new(x, y, Vector2::new(0.0, 0.0), max_vel, movement_speed, hitbox),
            texture,
            grounded: false,
        }
    }

    pub fn handle_input(&mut self, rl: &raylib::RaylibHandle) {
        let transform = &mut self.transform;
        /* ---------------------------------- input --------------------------------- */
        if self.grounded
            && ((rl.is_key_pressed(KEY_W) || rl.is_key_pressed(KEY_SPACE))
            && (transform.vel.y >= -transform.max_vel.y))
        {
            transform.vel.y -= transform.movement_speed.y;
        }

        if rl.is_key_down(KEY_A) && (transform.vel.x >= -transform.max_vel.x) {
            transform.vel.x -= transform.movement_speed.x;
        }

        if !self.grounded
            && (rl.is_key_down(KEY_S) && (transform.vel.y <= transform.max_vel.y))
        {
            transform.vel.y += transform.movement_speed.y;
        }

        if rl.is_key_down(KEY_D) && (transform.vel.x <= transform.max_vel.x) {
            transform.vel.x += transform.movement_speed.x;
        }

        /* -------------------------------- friction -------------------------------- */
        if transform.vel.x < 0.0 {
            transform.vel.x += 0.5;
        }
        if transform.vel.x > 0.0 {
            transform.vel.x -= 0.5;
        }

        if transform.vel.y < 0.0 {
            transform.vel.y += 0.5;
        }
        if transform.vel.y > 0.0 {
            transform.vel.y -= 0.5;
        }

        /* ----------------------------- updating player ---------------------------- */
        //gravity
        if !self.grounded {
            transform.vel.y += 0.75;
        } else if transform.vel.y >= 0.0 {
            transform.vel.y = 0.0;
        }

        transform.x += transform.vel.x as i32;
        transform.y += transform.vel.y as i32;

        if transform.x < 0 {
            transform.x = 0;
        }
        if transform.y < 0 {
            transform.y = 0;
        }
        if transform.x >= 310 {
            transform.x = 310;
        }
        if transform.y >= 170 {
            transform.y = 170;
        }

        transform.hitbox.rect.x = transform.x as f32 + transform.hitbox.offset.x;
        transform.hitbox.rect.y = transform.y as f32 + transform.hitbox.offset.y;
    }

    pub fn check_grounded(&mut self, map: &Map) {
        let sh = self.transform.hitbox.rect;
        self.grounded = match check_tile_at(
            map,
            ((sh.x + sh.width / 2.0) / 16.0) as i32,
            ((sh.y + sh.height) / 16.0) as i32,
        ) {
            None => { false }
            Some(_tile_id) => { true }
        };
    }

    pub fn draw(
        &self,
        draw_handle: &mut raylib::prelude::RaylibTextureMode<'_, raylib::prelude::RaylibDrawHandle<'_>>,
        player_tex: &Texture2D,
        is_debug: bool,
    ) {
        draw_handle.draw_texture_ex(
            player_tex,
            raylib::prelude::Vector2::new(self.transform.x as f32, self.transform.y as f32),
            0.0,
            1.0,
            Color::WHITE,
        );
        if is_debug {
            draw_handle.draw_rectangle_lines_ex(self.transform.hitbox.rect, 1, Color::RED);
        }
    }
}
