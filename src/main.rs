use raylib::prelude::*;

mod player;
mod types;
use player::*;

fn main() {
    /* -------------------------------------------------------------------------- */
    /*                                    init                                    */
    /* -------------------------------------------------------------------------- */

    /* ------------------------------- raylib init ------------------------------ */
    let screen_width = 1280;
    let screen_height = 720;

    let virt_screen_width = 320;
    let virt_screen_height = 180;

    let virt_ratio: f32 = (screen_width as f32) / (virt_screen_width as f32);

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Yet Another Retro Game")
        .build();

    rl.set_target_fps(60);

    let mut screen_camera = Camera2D {
        target: Vector2::new(screen_width as f32 / 2.0, screen_height as f32 / 2.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut world_camera = Camera2D {
        target: Vector2::new(screen_width as f32 / 2.0, screen_height as f32 / 2.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut virt_screen: RenderTexture2D = rl
        .load_render_texture(&thread, virt_screen_width, virt_screen_height)
        .unwrap();
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        virt_screen.texture.width as f32,
        -virt_screen.texture.height as f32,
    );
    let dest_rec = Rectangle::new(
        -virt_ratio,
        -virt_ratio,
        screen_width as f32 + (virt_ratio * 2.0),
        screen_height as f32 + (virt_ratio * 2.0),
    );

    /* -------------------------------- game init ------------------------------- */
    let mut player: Player = Player::new(0, 0, Vector2::new(3.0, 3.0), Vector2::new(1.0, 1.0), Image::load_image("resources/player.png").unwrap());
    let player_tex = rl.load_texture_from_image(&thread, &player.texture).unwrap();

    /* -------------------------------------------------------------------------- */
    /*                                  main loop                                 */
    /* -------------------------------------------------------------------------- */
    while !rl.window_should_close() {
        /* --------------------------------- update --------------------------------- */
        player.handle_input(&rl);

        /* ------------------------------ camera stuff ------------------------------ */
        world_camera.target.x = screen_camera.target.x;
        screen_camera.target.x -= world_camera.target.x;
        screen_camera.target.x *= virt_ratio;

        world_camera.target.y = screen_camera.target.y;
        screen_camera.target.y -= world_camera.target.y;
        screen_camera.target.y *= virt_ratio;

        /* --------------------------------- drawing -------------------------------- */
        let mut draw_handle = rl.begin_drawing(&thread);

        let mut texture_mode = draw_handle.begin_texture_mode(&thread, &mut virt_screen);
        let mut w_mode_2d = texture_mode.begin_mode2D(world_camera);
        w_mode_2d.clear_background(Color::WHITE);
        drop(w_mode_2d);
        player.draw(&mut texture_mode, &player_tex);

        drop(texture_mode);

        draw_handle.clear_background(Color::MAGENTA);
        let mut s_mode_2d = draw_handle.begin_mode2D(screen_camera);
        s_mode_2d.draw_texture_pro(
            &virt_screen,
            source_rec,
            dest_rec,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
        drop(s_mode_2d);
    }

    /* -------------------------------------------------------------------------- */
    /*                                   cleanup                                  */
    /* -------------------------------------------------------------------------- */
    drop(virt_screen);
    drop(player_tex);
}
