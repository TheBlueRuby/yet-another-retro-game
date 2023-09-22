use level::{load_level, load_tileset};
use raylib::prelude::*;
use tiled::Map;

mod player;
mod types;
mod level;

use player::*;
use types::*;

fn main() {
    /* -------------------------------------------------------------------------- */
    /*                                    init                                    */
    /* -------------------------------------------------------------------------- */
    let is_debug = cfg!(debug_assertions);

    /* ------------------------------- raylib init ------------------------------ */

    let (
            virt_ratio,
            (mut raylib_handle, raylib_thread),
            mut screen_camera,
            mut world_camera,
            mut virtual_screen_texture,
            virtual_screen_rec,
            window_rec
    ) = init(
        1280,
        720,
        "Yet Another Retro Game",
        320,
        180
    );

    /* -------------------------------- game init ------------------------------- */
    let mut player: Player = Player::new(
        16,
        16,
        Vector2::new(3.0, 5.0),
        Vector2::new(1.0, 1.0),
        Hitbox::new(
            Vector2::new(2.0, 0.0),
            Rectangle::new(0.0, 0.0, 13.0, 16.0),
        ),
        Image::load_image("resources/images/player.png").unwrap(),
    );

    let player_tex = raylib_handle
        .load_texture_from_image(&raylib_thread, &player.texture)
        .unwrap();

    let level = load_level("resources/levels/test.tmx");
    let tileset = load_tileset("resources/levels/monochrome.tsx");
    let tileset_tex = raylib_handle.load_texture(
        &raylib_thread,
        tileset.image.clone().unwrap().source.to_str().unwrap(),
    ).unwrap();

    /* -------------------------------------------------------------------------- */
    /*                                  main loop                                 */
    /* -------------------------------------------------------------------------- */
    while !raylib_handle.window_should_close() {
        /* --------------------------------- update --------------------------------- */
        player.handle_input(&raylib_handle);

        /* ------------------------------ camera stuff ------------------------------ */
        world_camera.target.x = screen_camera.target.x;
        screen_camera.target.x -= world_camera.target.x;
        screen_camera.target.x *= virt_ratio;

        world_camera.target.y = screen_camera.target.y;
        screen_camera.target.y -= world_camera.target.y;
        screen_camera.target.y *= virt_ratio;

        /* --------------------------------- physics -------------------------------- */
        player.check_grounded(&level);

        /* --------------------------------- drawing -------------------------------- */
        let mut draw_handle = raylib_handle.begin_drawing(&raylib_thread);

        {// drawing to render texture
            let mut virtual_screen = draw_handle.begin_texture_mode(&raylib_thread, &mut virtual_screen_texture);

            draw(&mut virtual_screen, &player, &player_tex, &level, &tileset_tex, is_debug);
        }

        // rendering texture to screen
        draw_handle.clear_background(Color::MAGENTA);

        let mut screen = draw_handle.begin_mode2D(screen_camera);
        screen.draw_texture_pro(
            &virtual_screen_texture,
            virtual_screen_rec,
            window_rec,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }

    /* -------------------------------------------------------------------------- */
    /*                                   cleanup                                  */
    /* -------------------------------------------------------------------------- */
    drop(virtual_screen_texture);
    drop(player_tex);
}

fn init(window_width: i32, window_height: i32, window_title: &str, virtual_screen_width: u32, virtual_screen_height: u32) -> (f32, (RaylibHandle, RaylibThread), Camera2D, Camera2D, RenderTexture2D, Rectangle, Rectangle) {
    let virt_ratio: f32 = (window_width as f32) / (virtual_screen_width as f32);

    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title(window_title)
        .build();

    raylib_handle.set_target_fps(60);

    let screen_camera = Camera2D {
        target: Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let world_camera = Camera2D {
        target: Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let virtual_screen_texture: RenderTexture2D = raylib_handle
        .load_render_texture(&raylib_thread, virtual_screen_width, virtual_screen_height)
        .unwrap();

    let virtual_screen_rec = Rectangle::new(
        0.0,
        0.0,
        virtual_screen_texture.texture.width as f32,
        -virtual_screen_texture.texture.height as f32,
    );
    let window_rec = Rectangle::new(
        -virt_ratio,
        -virt_ratio,
        window_width as f32 + (virt_ratio * 2.0),
        window_height as f32 + (virt_ratio * 2.0),
    );

    
    (virt_ratio, (raylib_handle, raylib_thread), screen_camera, world_camera, virtual_screen_texture, virtual_screen_rec, window_rec)
}

fn draw(
    draw_handle: &mut RaylibTextureMode<RaylibDrawHandle>,
    player: &Player,
    player_tex: &Texture2D,
    level: &Map,
    tileset_tex: &Texture2D,
    is_debug: bool,
) {
    draw_handle.clear_background(Color::WHITE);

    level::draw_tiles(draw_handle, level, tileset_tex);
    player.draw(draw_handle, player_tex, is_debug);
}
