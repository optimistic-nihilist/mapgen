mod fundamentals;
mod maptools;
mod procgen;
mod utils;
use fundamentals::{COLS, ROWS, TILESIZE, WINH, WINW};
use macroquad::prelude::*;
use maptools::{new_map, randomize_map, Map, TileType};
use procgen::bsp_tree::BSPTreeGenerator;
use procgen::cellular_automata::CellularAutomataGenerator;
use procgen::room_placement::RoomPlacementGenerator;
use procgen::rwalk::RandomWalkGenerator;
use procgen::tunneling::TunnelingGenerator;
use std::collections::HashMap;

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
                col as f32 * TILESIZE as f32,
                row as f32 * TILESIZE as f32,
                WHITE,
                tiles.get(&curr_type).unwrap().clone(),
            );
        }
    }
}

fn render_help_full(params: TextParams) {
    const HELP_TEXT: [&str; 4] = [
        "[r] - randomize map  [1] - tunneling       [5] - room placement",
        "[c] - clear map      [2] - BSP             [6] - maze with rooms",
        "[f] - toggle FPS     [3] - random walk     [LShift+num] - frenzy",
        "[h] - toggle help    [4] - cell. automata  [ESC] - quit",
    ];
    for (idx, row) in HELP_TEXT.iter().enumerate() {
        draw_text_ex(
            row,
            10.0,
            WINH as f32 - (HELP_TEXT.len() as f32 - 1.0 - idx as f32) * 19.0 - 16.0,
            params,
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // seed PRNG
    rand::srand(macroquad::miniquad::date::now() as _);

    // load texture tile sheet
    let texture: Texture2D = load_texture("assets/tilesheet_colored.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    // setup DrawTextureParams for individual tile types
    let tile_wall: DrawTextureParams = DrawTextureParams {
        source: Some(Rect::new(
            0.0 * TILESIZE as f32,
            0.0,
            TILESIZE as f32,
            TILESIZE as f32,
        )),
        ..Default::default()
    };
    let tile_floor: DrawTextureParams = DrawTextureParams {
        source: Some(Rect::new(
            1.0 * TILESIZE as f32,
            0.0,
            TILESIZE as f32,
            TILESIZE as f32,
        )),
        ..Default::default()
    };
    let tile_hero: DrawTextureParams = DrawTextureParams {
        source: Some(Rect::new(
            2.0 * TILESIZE as f32,
            0.0,
            TILESIZE as f32,
            TILESIZE as f32,
        )),
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

    // create initial empty map
    let mut map = new_map(TileType::Floor);

    let mut show_help = true;
    let mut show_fps = true;

    // main loop
    loop {
        let bg = Color::from_rgba(40, 40, 40, 255);
        clear_background(bg);

        render_map(&tiles, texture, &map);

        if show_fps {
            draw_text_ex(&format!("FPS: {}", get_fps()), 10.0, 26.0, font_params);
        }

        if show_help {
            render_help_full(font_params);
        } else {
            let mut gray_text = font_params.clone();
            gray_text.color = Color::from_rgba(160, 160, 160, 255);
            draw_text_ex("[h]", 10.0, WINH as f32 - 16.0, gray_text);
        }

        if is_key_pressed(KeyCode::H) {
            show_help = !show_help;
        }
        if is_key_pressed(KeyCode::F) {
            show_fps = !show_fps;
        }
        if is_key_pressed(KeyCode::C) {
            map = new_map(TileType::Floor);
        }
        if is_key_pressed(KeyCode::R) {
            map = randomize_map();
        }
        if is_key_pressed(KeyCode::Key1) {
            map = TunnelingGenerator::generate_map(6, 16, 30);
        }
        if is_key_pressed(KeyCode::Key2) {
            map = BSPTreeGenerator::generate_map();
        }
        if is_key_pressed(KeyCode::Key3) {
            map = RandomWalkGenerator::generate_map();
        }
        if is_key_pressed(KeyCode::Key4) {
            map = CellularAutomataGenerator::generate_map();
        }
        if is_key_pressed(KeyCode::Key5) {
            map = RoomPlacementGenerator::generate_map();
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::LeftShift) {
            if is_key_down(KeyCode::R) {
                map = randomize_map();
            }
            if is_key_down(KeyCode::Key1) {
                map = TunnelingGenerator::generate_map(6, 16, 30);
            }
            if is_key_down(KeyCode::Key2) {
                map = BSPTreeGenerator::generate_map();
            }
            if is_key_down(KeyCode::Key3) {
                map = RandomWalkGenerator::generate_map();
            }
            if is_key_down(KeyCode::Key4) {
                map = CellularAutomataGenerator::generate_map();
            }
            if is_key_down(KeyCode::Key5) {
                map = RoomPlacementGenerator::generate_map();
            }
        }

        next_frame().await
    }
}
