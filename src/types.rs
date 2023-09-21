use raylib::prelude::{Rectangle, Vector2};

#[derive(Default, Debug, Clone, Copy)]
pub struct Hitbox {
    pub offset: Vector2,
    pub rect: Rectangle,
}

impl Hitbox {
    pub fn new(offset: Vector2, rect: Rectangle) -> Self {
        Self { offset, rect }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub vel: Vector2,
    pub max_vel: Vector2,
    pub mvmt_spd: Vector2,
    pub hitbox: Hitbox,
}

impl Transform {
    pub fn new(
        x: i32,
        y: i32,
        vel: Vector2,
        max_vel: Vector2,
        mvmt_spd: Vector2,
        hitbox: Hitbox,
    ) -> Self {
        Self {
            x,
            y,
            vel,
            max_vel,
            mvmt_spd,
            hitbox,
        }
    }
}
