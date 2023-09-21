use raylib::prelude::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub vel: Vector2,
    pub mv: Vector2,
    pub sp: Vector2
}

impl Entity {
    pub fn new(
        x: i32,
        y: i32,
        vel: Vector2,
        mv: Vector2,
        sp: Vector2,
    ) -> Self {
        Entity {
            x,
            y,
            vel,
            mv,
            sp,
        }
    }
}
