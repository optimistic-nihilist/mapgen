use macroquad::rand;
use std::ops::Range;

pub fn randr(r: Range<i32>) -> i32 {
    rand::gen_range::<i32>(r.start, r.end)
}
