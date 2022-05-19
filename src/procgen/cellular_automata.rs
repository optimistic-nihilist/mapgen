use crate::{fundamentals::*, maptools::*, utils::*};

const DEATH_LIMIT: i32 = 3;
const BIRTH_LIMIT: i32 = 4;

fn randomize_map_seal_edges(m: &mut Map) {
    for y in 0..ROWS {
        for x in 0..COLS {
            if (x == 0) | (x == COLS - 1) | (y == 0) | (y == ROWS - 1) {
                m[y as usize][x as usize] = TileType::Wall as i32;
            } else {
                m[y as usize][x as usize] = match randr(0..100) {
                    0..=32 => TileType::Wall as i32,
                    _ => TileType::Floor as i32,
                };
            }
        }
    }
}

pub fn evolve_map(m: &mut Map) {
    for y in 0..ROWS {
        for x in 0..COLS {
            let neighbor_count = count_alive_neighbors(m, x, y);
            if m[y as usize][x as usize] == TileType::Wall as i32 {
                if neighbor_count < DEATH_LIMIT {
                    m[y as usize][x as usize] = TileType::Floor as i32
                }
            } else {
                if BIRTH_LIMIT < neighbor_count {
                    m[y as usize][x as usize] = TileType::Wall as i32
                }
            }
        }
    }
}

fn count_alive_neighbors(m: &Map, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            let nx = x + i;
            let ny = y + j;
            if (nx < 0) | (nx >= COLS) | (ny < 0) | (ny >= ROWS) {
                count += 1;
            } else if m[ny as usize][nx as usize] == TileType::Wall as i32 {
                count += 1;
            }
        }
    }
    count
}
fn get_random_cave_size(m: &mut Map) -> i32 {
    // get random floor starting position for DFS
    let mut rx = randr(0..COLS);
    let mut ry = randr(0..ROWS);
    while m[ry as usize][rx as usize] != TileType::Floor as i32 {
        rx = randr(0..COLS);
        ry = randr(0..ROWS);
    }
    let mut visited = new_map(TileType::Wall);
    dfs(rx, ry, &mut visited, m);

    for y in 0..ROWS {
        for x in 0..COLS {
            if visited[y as usize][x as usize] == 1 {
                m[y as usize][x as usize] = TileType::Floor as i32;
            } else {
                m[y as usize][x as usize] = TileType::Wall as i32;
            }
        }
    }

    let num_floor: i32 = visited
        .iter()
        .flat_map(|x: &[i32; COLS as usize]| x.iter())
        .sum();

    num_floor
}

fn dfs(x: i32, y: i32, v: &mut Map, m: &Map) {
    v[y as usize][x as usize] = TileType::Floor as i32;
    if is_valid(x - 1, y, v, m) {
        dfs(x - 1, y, v, m);
    }
    if is_valid(x + 1, y, v, m) {
        dfs(x + 1, y, v, m);
    }
    if is_valid(x, y - 1, v, m) {
        dfs(x, y - 1, v, m);
    }
    if is_valid(x, y + 1, v, m) {
        dfs(x, y + 1, v, m);
    }
}

fn is_valid(x: i32, y: i32, v: &Map, m: &Map) -> bool {
    if (x < 0) | (x >= COLS) | (y < 0) | (y >= ROWS) {
        return false;
    }
    if v[y as usize][x as usize] == TileType::Floor as i32 {
        return false;
    }
    if m[y as usize][x as usize] == TileType::Wall as i32 {
        return false;
    }
    true
}

fn generate_caves(m: &mut Map) {
    randomize_map_seal_edges(m);
    for _ in 0..15 {
        evolve_map(m);
    }
}

pub struct CellularAutomataGenerator {}
impl CellularAutomataGenerator {
    pub fn generate_map() -> Map {
        let mut map = new_map(TileType::Wall);
        generate_caves(&mut map);
        while get_random_cave_size(&mut map) < 1000 {
            generate_caves(&mut map);
        }
        map
    }
}
