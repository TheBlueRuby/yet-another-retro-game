use raylib::prelude::{Camera2D, Color, RaylibDraw, RaylibDrawHandle, RaylibMode2DExt, RaylibTextureModeExt, Rectangle, RenderTexture2D, Texture2D, Vector2};
use raylib::RaylibThread;
use tiled::Map;
use crate::level;
use crate::player::Player;

pub fn move_cameras(world_camera: &mut Camera2D, screen_camera: &mut Camera2D, virt_ratio: f32) {
    world_camera.target.x = screen_camera.target.x;
    screen_camera.target.x -= world_camera.target.x;
    screen_camera.target.x *= virt_ratio;

    world_camera.target.y = screen_camera.target.y;
    screen_camera.target.y -= world_camera.target.y;
    screen_camera.target.y *= virt_ratio;
}
pub fn draw(
    raylib_thread: &RaylibThread,
    draw_handle: &mut RaylibDrawHandle,
    virtual_screen_texture: &mut RenderTexture2D,
    player: &Player,
    player_tex: &Texture2D,
    level: &Map,
    tileset_tex: &Texture2D,
    is_debug: bool,
) {
    let mut virtual_screen = draw_handle.begin_texture_mode(raylib_thread, virtual_screen_texture);
    virtual_screen.clear_background(Color::WHITE);

    level::draw_tiles(&mut virtual_screen, level, tileset_tex);
    player.draw(&mut virtual_screen, player_tex, is_debug);
}

pub fn render(
    draw_handle: &mut RaylibDrawHandle,
    screen_camera: &Camera2D,
    virtual_screen_texture: &RenderTexture2D,
    virtual_screen_rec: Rectangle,
    window_rec: Rectangle,
) {
    draw_handle.clear_background(Color::MAGENTA);
    let mut screen = draw_handle.begin_mode2D(screen_camera);
    screen.draw_texture_pro(
        virtual_screen_texture,
        virtual_screen_rec,
        window_rec,
        Vector2 { x: 0.0, y: 0.0 },
        0.0,
        Color::WHITE,
    );
}