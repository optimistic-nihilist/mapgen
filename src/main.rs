use std::collections::HashMap;

use macroquad::prelude::*;

pub const WINW: i32 = 1280;
pub const WINH: i32 = 800;
pub const TILESIZE: i32 = 16;
pub const NUM_SPRITES: usize = 3;
pub const COLS: i32 = WINW / TILESIZE;
pub const ROWS: i32 = WINH / TILESIZE;

fn window_conf() -> Conf {
    Conf {
        window_title: "macroquad-test".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: WINW,
        window_height: WINH,
        ..Default::default()
    }
}

fn randomize_map() -> Map {
    let mut map = [[0; COLS as usize]; ROWS as usize];
    for row in 0..ROWS {
        for col in 0..COLS {
            map[row as usize][col as usize] = rand::gen_range::<i32>(0, 2);
        }
    }
    map
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum TileType {
    Wall,
    Floor,
    Hero,
}

fn render_map(tiles: &HashMap<TileType, DrawTextureParams>, texture: Texture2D, m: &Map) {
    for row in 0..ROWS {
        for col in 0..COLS {
            let curr_tile = m[row as usize][col as usize];
            let curr_type = match curr_tile {
                0 => TileType::Wall,
                1 => TileType::Floor,
                _ => TileType::Hero,
            };
            draw_texture_ex(
                texture,
                col as f32 * 16.0,
                row as f32 * 16.0,
                WHITE,
                tiles.get(&curr_type).unwrap().clone(),
            );
        }
    }
}

type Map = [[i32; COLS as usize]; ROWS as usize];

#[macroquad::main(window_conf)]
async fn main() {
    // seed PRNG
    rand::srand(macroquad::miniquad::date::now() as _);

    // load texture tile sheet
    let texture: Texture2D = load_texture("assets/tilesheet_colored.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    // setup DrawTextureParams for individual tile types
    let tile_wall: DrawTextureParams = DrawTextureParams {
        source: Some(Rect::new(0.0, 0.0, 16.0, 16.0)),
        ..Default::default()
    };
    let tile_floor: DrawTextureParams = DrawTextureParams {
        source: Some(Rect::new(16.0, 0.0, 16.0, 16.0)),
        ..Default::default()
    };
    let tile_hero: DrawTextureParams = DrawTextureParams {
        source: Some(Rect::new(32.0, 0.0, 16.0, 16.0)),
        ..Default::default()
    };

    // store each tiles' DrawTextureParams with their respective TileType as key
    let tiles: HashMap<TileType, DrawTextureParams> = HashMap::from([
        (TileType::Wall, tile_wall),
        (TileType::Floor, tile_floor),
        (TileType::Hero, tile_hero),
    ]);

    // load font
    let font: Font = load_ttf_font("assets/Hack-Regular.ttf").await.unwrap();
    let font_params: TextParams = TextParams {
        font,
        font_size: 16,
        ..Default::default()
    };

    // create initial map
    let mut map = randomize_map();

    // main loop
    loop {
        let bg = Color::from_rgba(40, 40, 40, 255);
        clear_background(bg);

        render_map(&tiles, texture, &map);
        draw_text_ex(&format!("FPS: {}", get_fps()), 16.0, 32.0, font_params);

        if is_key_down(KeyCode::R) {
            map = randomize_map();
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
