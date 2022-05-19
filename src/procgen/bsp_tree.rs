use crate::{fundamentals::*, maptools::*, utils::*};

#[derive(Clone, Debug)]
struct BSPNode {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    room: Option<Room>,
    left_child: Option<Box<BSPNode>>,
    right_child: Option<Box<BSPNode>>,
}

impl BSPNode {
    fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        room: Option<Room>,
        left: Option<Box<BSPNode>>,
        right: Option<Box<BSPNode>>,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            room,
            left_child: left,
            right_child: right,
        }
    }

    fn split(&mut self) -> bool {
        if self.left_child.is_some() && self.right_child.is_some() {
            // node is already split
            return false;
        }

        let mut hsplit = true;
        let ratio = self.w as f32 / self.h as f32;
        if ratio > 1.25 {
            hsplit = false;
        }

        let max = match hsplit {
            true => self.h - BSPTREE_LEAF_MIN_SIZE,
            false => self.w - BSPTREE_LEAF_MIN_SIZE,
        };

        if max <= BSPTREE_LEAF_MIN_SIZE {
            // node too small to split further
            return false;
        }

        let split = randr(BSPTREE_LEAF_MIN_SIZE..max);

        if hsplit {
            self.left_child = Some(Box::new(BSPNode::new(
                self.x, self.y, self.w, split, None, None, None,
            )));
            self.right_child = Some(Box::new(BSPNode::new(
                self.x,
                self.y + split,
                self.w,
                self.h - split,
                None,
                None,
                None,
            )));
        } else {
            self.left_child = Some(Box::new(BSPNode::new(
                self.x, self.y, split, self.h, None, None, None,
            )));
            self.right_child = Some(Box::new(BSPNode::new(
                self.x + split,
                self.y,
                self.w - split,
                self.h,
                None,
                None,
                None,
            )));
        }

        true
    }
}

fn carve_leafs(curr: &mut BSPNode, map: &mut Map) {
    if curr.left_child.is_some() || curr.right_child.is_some() {
        if curr.left_child.is_some() {
            carve_leafs(curr.left_child.as_mut().unwrap(), map);
        }
        if curr.right_child.is_some() {
            carve_leafs(curr.right_child.as_mut().unwrap(), map);
        }
        if let (Some(l), Some(r)) = (curr.left_child.as_mut(), curr.right_child.as_mut()) {
            let lroom = get_room(l).unwrap();
            let rroom = get_room(r).unwrap();

            let (lx, ly) = lroom.center();
            let (rx, ry) = rroom.center();

            if randr(0..1) == 1 {
                carve_horz_tunnel(map, lx, rx, ly);
                carve_vert_tunnel(map, ly, ry, rx);
            } else {
                carve_vert_tunnel(map, ly, ry, lx);
                carve_horz_tunnel(map, lx, rx, ry);
            }
        }
    } else {
        if curr.room.is_none() {
            let w = randr(BSPTREE_ROOM_MIN_SIZE..std::cmp::min(BSPTREE_ROOM_MAX_SIZE, curr.w - 1));
            let h = randr(BSPTREE_ROOM_MIN_SIZE..std::cmp::min(BSPTREE_ROOM_MAX_SIZE, curr.h - 1));
            let x = randr(curr.x..curr.x + (curr.w - 1) - w);
            let y = randr(curr.y..curr.y + (curr.h - 1) - h);

            curr.room = Some(Room::new(x, y, w, h));
            curr.room.unwrap().carve(map);
        }
    }
}

fn get_room(curr: &mut BSPNode) -> Option<Room> {
    if let Some(r) = curr.room {
        return Some(r);
    }
    if curr.left_child.is_some() {
        return get_room(curr.left_child.as_mut().unwrap());
    }
    if curr.right_child.is_some() {
        return get_room(curr.right_child.as_mut().unwrap());
    }
    None
}

fn split_until_fail(curr: &mut BSPNode) {
    if !curr.split() {
        return;
    }
    split_until_fail(curr.left_child.as_mut().unwrap());
    split_until_fail(curr.right_child.as_mut().unwrap());
}

pub struct BSPTreeGenerator {}

impl BSPTreeGenerator {
    pub fn generate_map() -> Map {
        let mut map = new_map(TileType::Wall);
        let mut root = BSPNode::new(1, 1, COLS, ROWS, None, None, None);
        split_until_fail(&mut root);
        carve_leafs(&mut root, &mut map);
        map
    }
}
