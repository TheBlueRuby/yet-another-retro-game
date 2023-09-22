use raylib::{
    prelude::{RaylibDraw, Rectangle, Vector2},
    texture::Texture2D,
};
use tiled::{Loader, Map, Tileset};

pub fn load_level(map_path: &str) -> Map {
    let mut loader = Loader::new();
    loader.load_tmx_map(map_path).unwrap()
}

pub fn load_tileset(tileset_path: &str) -> Tileset {
    let mut loader = Loader::new();
    loader.load_tsx_tileset(tileset_path).unwrap()
}

pub fn draw_tiles(
    draw_handle: &mut raylib::prelude::RaylibTextureMode<'_, raylib::prelude::RaylibDrawHandle<'_>>,
    map: &Map,
    tileset_texture: &Texture2D,
) {
    let tile_layer = map.get_layer(0).unwrap().as_tile_layer().unwrap();
    let map_tileset = &map.tilesets()[0];

    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            let tile_id = tile_layer.get_tile(x, y).unwrap().id();
            let tile_dest = Rectangle::new(
                (x * map.tile_width as i32) as f32,
                (y * map.tile_height as i32) as f32,
                map.tile_width as f32,
                map.tile_height as f32,
            );

            let mut tile_y = 0;
            let mut tile_x = 0;
            for _i in 0..tile_id {
                if tile_x == map_tileset.columns {
                    tile_x = 0;
                    tile_y += 1;
                }
                tile_x += 1;
            }

            let tile_source = Rectangle::new(
                (tile_x * map.tile_width) as f32,
                (tile_y * map.tile_height) as f32,
                map.tile_width as f32,
                map.tile_height as f32,
            );

            draw_handle.draw_texture_pro(
                tileset_texture,
                tile_source,
                tile_dest,
                Vector2::new(0.0, 0.0),
                0.0,
                raylib::prelude::Color::WHITE,
            );
        }
    }
}

pub fn check_tile_at(map: &Map, x: i32, y: i32) -> u32 {
    let tile_layer = map.get_layer(0).unwrap().as_tile_layer().unwrap();
    tile_layer.get_tile(x, y).unwrap().id()
}
