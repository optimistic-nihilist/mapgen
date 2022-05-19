use crate::{fundamentals::*, maptools::*, utils::*};

const SQUARE_ROOM_MIN_SIZE: i32 = 4;
const SQUARE_ROOM_MAX_SIZE: i32 = 8;
const CROSS_ROOM_MIN_SIZE: i32 = 6;
const CROSS_ROOM_MAX_SIZE: i32 = 16;
const CAVE_ROOM_MIN_SIZE: i32 = 8;
const CAVE_ROOM_MAX_SIZE: i32 = 14;

#[derive(Debug)]
struct Room {
    rect: Rect,
    tiles: Vec<TileType>,
}

impl Room {
    fn center(&self) -> (i32, i32) {
        (self.rect.w / 2, self.rect.h / 2)
    }
}

fn room_get_xy(x: i32, y: i32, cols: i32) -> i32 {
    y * cols + x
}

fn room_get_coord_from_idx(idx: i32, cols: i32) -> (i32, i32) {
    let x = idx % cols;
    let y = idx / cols;
    (x, y)
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct TileWithXYIdx {
    tile: TileType,
    x: i32,
    y: i32,
    idx: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ConnectionPoint {
    tile: TileWithXYIdx,
    loc: ConnectionPointLocation,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ConnectionPointLocation {
    Top,
    Right,
    Bottom,
    Left,
}

fn find_connection_points(r: &Room) -> Vec<ConnectionPoint> {
    // get all (x,y) coords for all tiles
    let tiles_with_idx: Vec<TileWithXYIdx> = r
        .tiles
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let (x, y) = room_get_coord_from_idx(i as i32, r.rect.w);
            return TileWithXYIdx {
                tile: *t,
                idx: i as i32,
                x,
                y,
            };
        })
        .collect();

    // find all floors, work only with those further
    let floor_tiles: Vec<TileWithXYIdx> = tiles_with_idx
        .into_iter()
        .filter(|&t| t.tile as i32 == TileType::Floor as i32)
        .collect();

    // find boundaries
    let min_x = floor_tiles.iter().min_by_key(|&t| t.x).unwrap().x;
    let max_x = floor_tiles.iter().max_by_key(|&t| t.x).unwrap().x;
    let min_y = floor_tiles.iter().min_by_key(|&t| t.y).unwrap().y;
    let max_y = floor_tiles.iter().max_by_key(|&t| t.y).unwrap().y;

    let mut boundaries: Vec<Vec<&TileWithXYIdx>> = Vec::new();
    boundaries.push(floor_tiles.iter().filter(|&t| t.x == min_x).collect());
    boundaries.push(floor_tiles.iter().filter(|&t| t.x == max_x).collect());
    boundaries.push(floor_tiles.iter().filter(|&t| t.y == min_y).collect());
    boundaries.push(floor_tiles.iter().filter(|&t| t.y == max_y).collect());

    // select random point from boundary points
    let mut connection_points: Vec<ConnectionPoint> = Vec::new();
    for (i, b) in boundaries.iter().enumerate() {
        let connection_idx = if b.len() <= 2 {
            randr(0..b.len() as i32)
        } else {
            randr(1..b.len() as i32 - 1)
        };

        let connection_point = b[connection_idx as usize];
        connection_points.push(ConnectionPoint {
            tile: *connection_point,
            loc: match i {
                0 => ConnectionPointLocation::Left,
                1 => ConnectionPointLocation::Right,
                2 => ConnectionPointLocation::Top,
                _ => ConnectionPointLocation::Bottom,
            },
        });
    }

    for c in &mut connection_points {
        c.tile.x = c.tile.x + r.rect.x;
        c.tile.y = c.tile.y + r.rect.y;
        c.tile.idx = get_xy_idx(c.tile.x, c.tile.y);
    }

    connection_points
}

fn get_xy_idx(x: i32, y: i32) -> i32 {
    y * COLS + x
}

fn generate_square_room(size_min: i32, size_max: i32) -> Room {
    let room_size = randr(size_min..size_max + 1);
    let mut tiles: Vec<TileType> = vec![TileType::Wall; (room_size * room_size) as usize];
    let rect = Rect {
        x: 0,
        y: 0,
        w: room_size,
        h: room_size,
    };
    for x in 0..room_size {
        for y in 0..room_size {
            tiles[room_get_xy(x, y, room_size) as usize] = TileType::Floor;
        }
    }

    Room { rect, tiles }
}

fn generate_rectangular_room(size_min: i32, size_max: i32) -> Room {
    let room_width = randr(size_min..size_max + 1);
    let room_height = randr(size_min..size_max + 1);
    let mut tiles: Vec<TileType> = vec![TileType::Wall; (room_width * room_height) as usize];
    let rect = Rect {
        x: 0,
        y: 0,
        w: room_width,
        h: room_height,
    };
    for x in 0..room_width {
        for y in 0..room_height {
            tiles[room_get_xy(x, y, room_width) as usize] = TileType::Floor;
        }
    }
    Room { rect, tiles }
}

fn generate_cross_room(size_min: i32, size_max: i32) -> Room {
    let mut r = generate_rectangular_room(size_min, size_max);
    let w_third = r.rect.w / 3;
    let h_third = r.rect.h / 3;
    for x in 0..w_third {
        for y in 0..h_third {
            r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
        }
        for y in r.rect.h - h_third..r.rect.h {
            r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
        }
    }
    for x in r.rect.w - w_third..r.rect.w {
        for y in 0..h_third {
            r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
        }
        for y in r.rect.h - h_third..r.rect.h {
            r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
        }
    }
    r
}

fn generate_circular_room(size_min: i32, size_max: i32) -> Room {
    let mut r = generate_square_room(size_min, size_max);
    let radius = r.rect.w / 2;
    let (cx, cy) = r.center();
    for x in 0..r.rect.w {
        for y in 0..r.rect.h {
            let dist = (((x - cx).pow(2) + (y - cy).pow(2)) as f64).sqrt();
            if dist.round() >= radius as f64 {
                r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
            }
        }
    }
    r
}

fn generate_cave_room(size_min: i32, size_max: i32) -> Room {
    let mut r = generate_square_room(size_min, size_max);

    generate_cave(&mut r);
    while get_random_cave_size(&mut r) < 80 {
        r = generate_square_room(size_min, size_max);
        generate_cave(&mut r);
    }
    r
}

fn generate_cave(r: &mut Room) {
    for x in 0..r.rect.w {
        for y in 0..r.rect.h {
            if x == 0 || x == r.rect.w || y == 0 || y == r.rect.h {
                r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
            } else {
                if randr(0..100) > 70 {
                    r.tiles[room_get_xy(x, y, r.rect.w) as usize] = TileType::Wall;
                }
            }
        }
    }
    for _ in 0..5 {
        evolve_map(r);
    }
}

fn evolve_map(r: &mut Room) {
    let death_limit = 3;
    let birth_limit = 4;

    for y in 0..r.rect.w {
        for x in 0..r.rect.h {
            let neighbor_count = count_alive_neighbors(r, x, y);
            let tile = &mut r.tiles[room_get_xy(x, y, r.rect.w) as usize];
            if *tile as i32 == TileType::Wall as i32 {
                if neighbor_count < death_limit {
                    *tile = TileType::Floor;
                }
            } else {
                if birth_limit < neighbor_count {
                    *tile = TileType::Wall;
                }
            }
        }
    }
}

fn count_alive_neighbors(r: &Room, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            let nx = x + i;
            let ny = y + j;
            if (nx < 0) | (nx >= r.rect.w) | (ny < 0) | (ny >= r.rect.h) {
                count += 1;
            } else if r.tiles[room_get_xy(nx, ny, r.rect.w) as usize] as i32
                == TileType::Wall as i32
            {
                count += 1;
            }
        }
    }
    count
}

fn get_random_cave_size(r: &mut Room) -> i32 {
    // count as 0 if less than 10 floors
    let mut num_floor: i32 = r.tiles.iter().map(|x| (*x) as i32).sum();
    if num_floor < 10 {
        return 0;
    }

    // get random floor starting position for DFS
    let mut rx = randr(0..r.rect.w);
    let mut ry = randr(0..r.rect.h);
    while r.tiles[room_get_xy(rx, ry, r.rect.w) as usize] as i32 != TileType::Floor as i32 {
        rx = randr(0..r.rect.w);
        ry = randr(0..r.rect.h);
    }
    let mut visited: Vec<i32> = Vec::with_capacity((r.rect.w * r.rect.h) as usize);
    for _ in r.tiles.iter() {
        visited.push(0);
    }
    dfs(rx, ry, &mut visited, r);

    for (pos, tile) in r.tiles.iter_mut().enumerate() {
        if visited[pos] == 1 {
            *tile = TileType::Floor;
        } else {
            *tile = TileType::Wall;
        }
    }

    num_floor = visited.iter().sum();
    num_floor
}

fn dfs(x: i32, y: i32, v: &mut [i32], r: &Room) {
    v[room_get_xy(x, y, r.rect.w) as usize] = 1;
    if is_valid(x - 1, y, v, r) {
        dfs(x - 1, y, v, r);
    }
    if is_valid(x + 1, y, v, r) {
        dfs(x + 1, y, v, r);
    }
    if is_valid(x, y - 1, v, r) {
        dfs(x, y - 1, v, r);
    }
    if is_valid(x, y + 1, v, r) {
        dfs(x, y + 1, v, r);
    }
}

fn is_valid(x: i32, y: i32, v: &[i32], r: &Room) -> bool {
    if (x < 0) | (x >= r.rect.w) | (y < 0) | (y >= r.rect.h) {
        return false;
    }
    if v[room_get_xy(x, y, r.rect.w) as usize] == 1 {
        return false;
    }
    if r.tiles[room_get_xy(x, y, r.rect.w) as usize] as i32 == TileType::Wall as i32 {
        return false;
    }

    true
}

/// place_room transposes the Room rect to map coordinates, and carves the Room tiles
fn place_room(r: &mut Room, m: &mut Map, xoff: i32, yoff: i32) -> bool {
    r.rect.x = xoff;
    r.rect.y = yoff;
    // first pass, check if new room is in bounds & no overlap
    for x in xoff - 1..xoff + r.rect.w + 1 {
        for y in yoff - 1..yoff + r.rect.h + 1 {
            if !in_bounds(x, y) {
                return false;
            }
            if m[y as usize][x as usize] == TileType::Floor as i32 {
                return false;
            }
        }
    }
    // second pass, place room if first pass was OK
    for x in xoff..xoff + r.rect.w {
        for y in yoff..yoff + r.rect.h {
            m[y as usize][x as usize] =
                r.tiles[room_get_xy(x - xoff, y - yoff, r.rect.w) as usize] as i32;
        }
    }
    true
}

const N_ROOM_TYPE: i32 = 5;
enum RoomType {
    Square,
    Rectangle,
    Cross,
    Circle,
    Cave,
}

fn generate_random_room() -> Room {
    let room_type: RoomType = match randr(0..N_ROOM_TYPE) {
        0 => RoomType::Square,
        1 => RoomType::Rectangle,
        2 => RoomType::Cross,
        3 => RoomType::Circle,
        _ => RoomType::Cave,
    };
    let r = match room_type {
        RoomType::Square => generate_square_room(SQUARE_ROOM_MIN_SIZE, SQUARE_ROOM_MAX_SIZE),
        RoomType::Rectangle => {
            generate_rectangular_room(SQUARE_ROOM_MIN_SIZE, SQUARE_ROOM_MAX_SIZE)
        }
        RoomType::Cross => generate_cross_room(CROSS_ROOM_MIN_SIZE, CROSS_ROOM_MAX_SIZE),
        RoomType::Circle => generate_circular_room(SQUARE_ROOM_MIN_SIZE, SQUARE_ROOM_MAX_SIZE),
        RoomType::Cave => generate_cave_room(CAVE_ROOM_MIN_SIZE, CAVE_ROOM_MAX_SIZE),
    };
    r
}

fn try_place_room(free_connection_points: &mut Vec<ConnectionPoint>, map: &mut Map) -> bool {
    // generate a random type room
    let mut r = generate_random_room();

    // select random connection point on starting room
    let cp = free_connection_points[randr(0..free_connection_points.len() as i32) as usize];

    // try to move room next to starting room connection point, depending on cp location
    let placement_offset = 4;
    let (tx, ty) = match cp.loc {
        ConnectionPointLocation::Top => (
            cp.tile.x - r.rect.w / 2,
            cp.tile.y - r.rect.h - placement_offset,
        ),
        ConnectionPointLocation::Right => (cp.tile.x + placement_offset, cp.tile.y - r.rect.h / 2),
        ConnectionPointLocation::Bottom => (cp.tile.x - r.rect.w / 2, cp.tile.y + placement_offset),
        ConnectionPointLocation::Left => (
            cp.tile.x - r.rect.w - placement_offset,
            cp.tile.y - r.rect.h / 2,
        ),
    };

    if !place_room(&mut r, map, tx, ty) {
        return false;
    }

    let mut new_room_connection_points = find_connection_points(&r);

    match cp.loc {
        ConnectionPointLocation::Top => {
            new_room_connection_points.retain(|&p| p.loc != ConnectionPointLocation::Bottom)
        }
        ConnectionPointLocation::Right => {
            new_room_connection_points.retain(|&p| p.loc != ConnectionPointLocation::Left)
        }
        ConnectionPointLocation::Bottom => {
            new_room_connection_points.retain(|&p| p.loc != ConnectionPointLocation::Top)
        }
        ConnectionPointLocation::Left => {
            new_room_connection_points.retain(|&p| p.loc != ConnectionPointLocation::Right)
        }
    }

    // connect rooms
    match cp.loc {
        ConnectionPointLocation::Top => {
            let tx = cp.tile.x;
            let mut ty = cp.tile.y;
            while map[ty as usize - 1][tx as usize] != TileType::Floor as i32 {
                map[ty as usize - 1][tx as usize] = TileType::Floor as i32;
                ty = ty - 1;
            }
        }
        ConnectionPointLocation::Bottom => {
            let tx = cp.tile.x;
            let mut ty = cp.tile.y;
            while map[ty as usize + 1][tx as usize] != TileType::Floor as i32 {
                map[ty as usize + 1][tx as usize] = TileType::Floor as i32;
                ty = ty + 1;
            }
        }
        ConnectionPointLocation::Left => {
            let mut tx = cp.tile.x;
            let ty = cp.tile.y;
            while map[ty as usize][tx as usize - 1] != TileType::Floor as i32 {
                map[ty as usize][tx as usize - 1] = TileType::Floor as i32;
                tx = tx - 1;
            }
        }
        ConnectionPointLocation::Right => {
            let mut tx = cp.tile.x;
            let ty = cp.tile.y;
            while map[ty as usize][tx as usize + 1] != TileType::Floor as i32 {
                map[ty as usize][tx as usize + 1] = TileType::Floor as i32;
                tx = tx + 1;
            }
        }
    }

    // remove used up connection point on previous room
    free_connection_points.retain(|&p| (p != cp));
    // add new room's free connection points
    free_connection_points.extend(new_room_connection_points);

    true
}

pub struct RoomPlacementGenerator {}

impl RoomPlacementGenerator {
    pub fn generate_map() -> Map {
        let mut map = new_map(TileType::Wall);

        // generate & place starting room in center
        let mut r1 = generate_random_room();
        let starting_room_x = COLS / 2 - r1.rect.w / 2;
        let starting_room_y = ROWS / 2 - r1.rect.h / 2;
        place_room(&mut r1, &mut map, starting_room_x, starting_room_y);

        // add starting room's connection points to vec containg all free connection points
        let mut free_connection_points = find_connection_points(&r1);

        let max_attempts = 100;
        let mut rooms_placed = 1;
        for _ in 0..max_attempts {
            if try_place_room(&mut free_connection_points, &mut map) {
                rooms_placed = rooms_placed + 1;
            }
        }

        // println!(
        //     "Placed {} rooms out of {} attempts.",
        //     rooms_placed, max_attempts
        // );

        // for c in free_connection_points {
        //     map[get_xy_idx(c.tile.x, c.tile.y) as usize] = TileType::Hero as i32;
        // }

        map
    }
}
