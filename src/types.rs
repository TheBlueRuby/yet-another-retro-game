#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) vel_x: f32,
    pub(crate) vel_y: f32,
    pub(crate) mv_x: f32,
    pub(crate) mv_y: f32,
    pub(crate) sp_x: f32,
    pub(crate) sp_y: f32,
}

impl Entity {
    pub fn new(
        x: i32,
        y: i32,
        vel_x: f32,
        vel_y: f32,
        mv_x: f32,
        mv_y: f32,
        sp_x: f32,
        sp_y: f32,
    ) -> Self {
        Entity {
            x,
            y,
            vel_x,
            vel_y,
            mv_x,
            mv_y,
            sp_x,
            sp_y,
        }
    }
}
