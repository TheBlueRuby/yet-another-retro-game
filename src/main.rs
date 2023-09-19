use raylib::prelude::*;

mod player;
mod types;
use player::*;

fn main() {
    /* ---------------------------------- init ---------------------------------- */
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Yet Another Retro Game")
        .build();

    rl.set_target_fps(60);

    let mut player: Player = Player::new(0, 0, 8, 8, 4, 4);

    /* -------------------------------- main loop ------------------------------- */
    while !rl.window_should_close() {
        player.handle_input(&mut rl);

        /* --------------------------------- drawing -------------------------------- */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw(d, player);
    }
}

fn draw(mut d: RaylibDrawHandle<'_>, player: Player) {
    d.draw_rectangle(player.transform.x, player.transform.y, 25, 25, Color::BLUE);
}
