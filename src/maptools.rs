use crate::fundamentals::*;
use crate::utils::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Hero,
}

pub type Map = [[i32; COLS as usize]; ROWS as usize];

pub fn new_map(fill_with: TileType) -> Map {
    [[fill_with as i32; COLS as usize]; ROWS as usize]
}

pub fn randomize_map() -> Map {
    let mut map = [[0; COLS as usize]; ROWS as usize];
    for row in 0..ROWS {
        for col in 0..COLS {
            map[row as usize][col as usize] = randr(0..2);
        }
    }
    map
}

pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Room {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            x2: x + w,
            y1: y,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let cx = (self.x1 + self.x2) / 2;
        let cy = (self.y1 + self.y2) / 2;
        (cx, cy)
    }

    pub fn overlaps(&self, other: Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn carve(&self, m: &mut Map) {
        for x in self.x1..self.x2 {
            for y in self.y1..self.y2 {
                m[y as usize][x as usize] = TileType::Floor as i32;
            }
        }
    }
}

pub fn carve_horz_tunnel(m: &mut Map, x1: i32, x2: i32, y: i32) {
    let min_x = std::cmp::min(x1, x2);
    let max_x = std::cmp::max(x1, x2);
    for x in min_x..max_x + 1 {
        m[y as usize][x as usize] = TileType::Floor as i32;
    }
}

pub fn carve_vert_tunnel(m: &mut Map, y1: i32, y2: i32, x: i32) {
    let min_y = std::cmp::min(y1, y2);
    let max_y = std::cmp::max(y1, y2);
    for y in min_y..max_y + 1 {
        m[y as usize][x as usize] = TileType::Floor as i32;
    }
}
