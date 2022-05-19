use crate::{fundamentals::*, maptools::*, utils::*};

#[derive(Copy, Clone, Debug)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn new(x: i32, y: i32, visited: &mut Map, visited_positions: &mut Vec<Pos>) -> Self {
        visited[y as usize][x as usize] = TileType::Floor as i32;
        visited_positions.push(Pos { x, y });
        Self { x, y }
    }

    fn step(&mut self, m: &mut Map, v: &mut Map, visited_positions: &mut Vec<Pos>) {
        let neighbors = get_neighbors(self.x, self.y, m);
        let mut valid_neighbors: Vec<Cell> = Vec::new();
        if let Some(n) = neighbors.north {
            valid_neighbors.push(n);
        }
        if let Some(e) = neighbors.east {
            valid_neighbors.push(e);
        }
        if let Some(s) = neighbors.south {
            valid_neighbors.push(s);
        }
        if let Some(w) = neighbors.west {
            valid_neighbors.push(w);
        }
        // backtrack if no valid neighbors
        if valid_neighbors.is_empty() {
            if visited_positions.is_empty() {
                return;
            }
            let prev_loc = visited_positions.pop().unwrap();
            (self.x, self.y) = (prev_loc.x, prev_loc.y);
            return;
        }

        let next = valid_neighbors[randr(0..valid_neighbors.len() as i32) as usize];
        (self.x, self.y) = (next.x, next.y);
        v[self.y as usize][self.x as usize] = TileType::Floor as i32;
        visited_positions.push(Pos {
            x: self.x,
            y: self.y,
        });
        m[self.y as usize][self.x as usize] = TileType::Floor as i32;
    }
}

struct Neighbors {
    north: Option<Cell>,
    east: Option<Cell>,
    south: Option<Cell>,
    west: Option<Cell>,
}

fn can_grow_tunnel(x: i32, y: i32, v: &Map) -> bool {
    if in_bounds(x, y) && v[y as usize][x as usize] == TileType::Wall as i32 {
        return true;
    }
    false
}

fn get_neighbors(x: i32, y: i32, m: &Map) -> Neighbors {
    let mut n: Option<Cell> = None;
    let mut e: Option<Cell> = None;
    let mut s: Option<Cell> = None;
    let mut w: Option<Cell> = None;

    if can_grow_tunnel(x, y - 1, m)
        && can_grow_tunnel(x, y - 2, m)
        && can_grow_tunnel(x - 1, y - 1, m)
        && can_grow_tunnel(x - 1, y - 2, m)
        && can_grow_tunnel(x + 1, y - 1, m)
        && can_grow_tunnel(x + 1, y - 2, m)
    {
        n = Some(Cell { x, y: y - 1 });
    }
    if can_grow_tunnel(x + 1, y, m)
        && can_grow_tunnel(x + 2, y, m)
        && can_grow_tunnel(x + 1, y - 1, m)
        && can_grow_tunnel(x + 2, y - 1, m)
        && can_grow_tunnel(x + 1, y + 1, m)
        && can_grow_tunnel(x + 2, y + 1, m)
    {
        e = Some(Cell { x: x + 1, y });
    }
    if can_grow_tunnel(x, y + 1, m)
        && can_grow_tunnel(x, y + 2, m)
        && can_grow_tunnel(x - 1, y + 1, m)
        && can_grow_tunnel(x - 1, y + 2, m)
        && can_grow_tunnel(x + 1, y + 1, m)
        && can_grow_tunnel(x + 1, y + 2, m)
    {
        s = Some(Cell { x, y: y + 1 });
    }
    if can_grow_tunnel(x - 1, y, m)
        && can_grow_tunnel(x - 2, y, m)
        && can_grow_tunnel(x - 2, y - 1, m)
        && can_grow_tunnel(x - 1, y - 1, m)
        && can_grow_tunnel(x - 2, y + 1, m)
        && can_grow_tunnel(x - 1, y + 1, m)
    {
        w = Some(Cell { x: x - 1, y });
    }

    Neighbors {
        north: n,
        east: e,
        south: s,
        west: w,
    }
}

fn count_neighbors(p: &Pos, v: &Map) -> i32 {
    let mut count = 0;
    if in_bounds(p.x - 1, p.y) && v[p.y as usize][p.x as usize - 1] == TileType::Floor as i32 {
        count = count + 1;
    }
    if in_bounds(p.x + 1, p.y) && v[p.y as usize][p.x as usize + 1] == TileType::Floor as i32 {
        count = count + 1;
    }
    if in_bounds(p.x, p.y - 1) && v[p.y as usize - 1][p.x as usize] == TileType::Floor as i32 {
        count = count + 1;
    }
    if in_bounds(p.x, p.y + 1) && v[p.y as usize + 1][p.x as usize] == TileType::Floor as i32 {
        count = count + 1;
    }
    count
}

fn trim_dead_ends(m: &mut Map) {
    // find dead ends - cells with exactly one neighbor
    let dead_ends: Vec<Pos> = m
        .iter()
        .flat_map(|x: &[i32; COLS as usize]| x.iter())
        .enumerate()
        .filter(|(_, &d)| d == 1)
        .map(|(idx, _)| {
            let x = idx as i32 % COLS;
            let y = idx as i32 / COLS;
            Pos { x, y }
        })
        .filter(|p| count_neighbors(p, m) == 1)
        .collect();
    for d in dead_ends {
        m[d.y as usize][d.x as usize] = TileType::Wall as i32;
    }
}

fn place_rooms(m: &mut Map) -> Vec<Room> {
    const ROOM_SIZE_MIN: i32 = 6;
    const ROOMS_SIZE_MAX: i32 = 16;
    let mut rooms: Vec<Room> = vec![];

    for _ in 0..20 {
        let w: i32 = randr(ROOM_SIZE_MIN..ROOMS_SIZE_MAX);
        let h: i32 = randr(ROOM_SIZE_MIN..ROOMS_SIZE_MAX);
        let x: i32 = randr(1..COLS - w);
        let y: i32 = randr(1..ROWS - h);

        let mut curr_room = Room::new(x, y, w, h);

        let mut overlaps = false;

        for room in &rooms {
            if curr_room.overlaps(*room) {
                overlaps = true;
                break;
            }
        }

        if !overlaps {
            // increase room rect size to allow for sparser room placement
            curr_room.carve(m);
            curr_room.x1 = curr_room.x1 - 3;
            curr_room.y1 = curr_room.y1 - 3;
            curr_room.x2 = curr_room.x2 + 3;
            curr_room.y2 = curr_room.y2 + 3;
            rooms.push(curr_room);
        }
    }

    rooms
}

fn connect_rooms(rooms: &mut Vec<Room>, m: &mut Map) {
    for room in rooms {
        // find possible connection points
        // = walls of the room which have a tunnel next to them

        // first shrink the room rect after placement
        room.x1 = room.x1 + 3;
        room.x2 = room.x2 - 3;
        room.y1 = room.y1 + 3;
        room.y2 = room.y2 - 3;

        // find the room walls
        let mut perimeter: Vec<Pos> = Vec::new();
        for x in room.x1..room.x2 {
            for y in room.y1..room.y2 {
                if x == room.x1 || x == room.x2 - 1 || y == room.y1 || y == room.y2 - 1 {
                    perimeter.push(Pos { x, y });
                }
            }
        }

        let mut possible_connection_points: Vec<Pos> = Vec::new();
        for p in perimeter {
            if in_bounds(p.x - 1, p.y)
                && in_bounds(p.x - 2, p.y)
                && m[p.y as usize][p.x as usize - 1] == TileType::Wall as i32
                && m[p.y as usize][p.x as usize - 2] == TileType::Floor as i32
            {
                possible_connection_points.push(p);
            }
            if in_bounds(p.x + 1, p.y)
                && in_bounds(p.x + 2, p.y)
                && m[p.y as usize][p.x as usize + 1] == TileType::Wall as i32
                && m[p.y as usize][p.x as usize + 2] == TileType::Floor as i32
            {
                possible_connection_points.push(p);
            }
            if in_bounds(p.x, p.y - 1)
                && in_bounds(p.x, p.y - 2)
                && m[p.y as usize - 1][p.x as usize] == TileType::Wall as i32
                && m[p.y as usize - 2][p.x as usize] == TileType::Floor as i32
            {
                possible_connection_points.push(p);
            }
            if in_bounds(p.x, p.y + 1)
                && in_bounds(p.x, p.y + 2)
                && m[p.y as usize + 1][p.x as usize] == TileType::Wall as i32
                && m[p.y as usize + 2][p.x as usize] == TileType::Floor as i32
            {
                possible_connection_points.push(p);
            }
        }

        if possible_connection_points.is_empty() {
            return;
        }

        let mut connection_points: Vec<Pos> = Vec::new();
        for _ in 0..2 {
            let connection_point = possible_connection_points
                [randr(0..possible_connection_points.len() as i32) as usize];
            connection_points.push(connection_point);
        }

        for p in connection_points {
            if in_bounds(p.x - 1, p.y)
                && in_bounds(p.x - 2, p.y)
                && m[p.y as usize][p.x as usize - 1] == TileType::Wall as i32
                && m[p.y as usize][p.x as usize - 2] == TileType::Floor as i32
            {
                m[p.y as usize][p.x as usize - 1] = TileType::Floor as i32;
                continue;
            }
            if in_bounds(p.x + 1, p.y)
                && in_bounds(p.x + 2, p.y)
                && m[p.y as usize][p.x as usize + 1] == TileType::Wall as i32
                && m[p.y as usize][p.x as usize + 2] == TileType::Floor as i32
            {
                m[p.y as usize][p.x as usize + 1] = TileType::Floor as i32;
                continue;
            }
            if in_bounds(p.x, p.y - 1)
                && in_bounds(p.x, p.y - 2)
                && m[p.y as usize - 1][p.x as usize] == TileType::Wall as i32
                && m[p.y as usize - 2][p.x as usize] == TileType::Floor as i32
            {
                m[p.y as usize - 1][p.x as usize] = TileType::Floor as i32;
                continue;
            }
            if in_bounds(p.x, p.y + 1)
                && in_bounds(p.x, p.y + 2)
                && m[p.y as usize + 1][p.x as usize] == TileType::Wall as i32
                && m[p.y as usize + 2][p.x as usize] == TileType::Floor as i32
            {
                m[p.y as usize + 1][p.x as usize] = TileType::Floor as i32;
                continue;
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

pub struct MazeGenerator {}
impl MazeGenerator {
    pub fn generate_map() -> Map {
        let mut map = new_map(TileType::Wall);
        let mut visited = new_map(TileType::Wall);
        let mut visited_positions: Vec<Pos> = Vec::new();

        let mut rooms = place_rooms(&mut map);

        // pick a random wall location
        let mut startx = COLS / 2;
        let mut starty = ROWS / 2;
        while map[starty as usize][startx as usize] != TileType::Wall as i32 {
            startx = randr(0..COLS);
            starty = randr(0..ROWS);
        }

        let mut c = Cell::new(startx, starty, &mut visited, &mut visited_positions);

        while !visited_positions.is_empty() {
            c.step(&mut map, &mut visited, &mut visited_positions);
        }

        for y in 0..ROWS {
            for x in 0..COLS {
                if visited[y as usize][x as usize] == TileType::Floor as i32 {
                    map[y as usize][x as usize] = TileType::Floor as i32;
                }
            }
        }

        // make maze passages sparser by trimming some dead ends
        for _ in 0..5 {
            trim_dead_ends(&mut map);
        }

        // connect rooms with passages
        connect_rooms(&mut rooms, &mut map);

        map
    }
}
