use crate::fundamentals::{COLS, ROWS};
use macroquad::rand;
use std::ops::Range;

pub fn randr(r: Range<i32>) -> i32 {
    rand::gen_range::<i32>(r.start, r.end)
}

pub fn in_bounds(x: i32, y: i32) -> bool {
    0 <= x && x < COLS && 0 <= y && y < ROWS
}
