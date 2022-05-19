use crate::fundamentals::*;
use crate::maptools::*;
use crate::utils::*;

pub struct TunnelingGenerator {}
impl TunnelingGenerator {
    pub fn generate_map(room_size_min: i32, room_size_max: i32, max_rooms: i32) -> Map {
        let mut map = new_map(TileType::Wall);
        let mut rooms: Vec<Room> = vec![];
        let mut num_rooms = 0;

        for _ in 0..max_rooms {
            let w = randr(room_size_min..room_size_max);
            let h = randr(room_size_min..room_size_max);
            let x = randr(1..COLS - w);
            let y = randr(1..ROWS - h);

            let curr_room = Room::new(x, y, w, h);
            let mut overlaps = false;

            for room in &rooms {
                if curr_room.overlaps(*room) {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                curr_room.carve(&mut map);

                let (curr_x, curr_y) = curr_room.center();

                if num_rooms != 0 {
                    let prev_room = rooms[num_rooms - 1];
                    let (prev_x, prev_y) = prev_room.center();

                    if randr(0..1) == 1 {
                        carve_horz_tunnel(&mut map, curr_x, prev_x, curr_y);
                        carve_vert_tunnel(&mut map, curr_y, prev_y, prev_x);
                    } else {
                        carve_vert_tunnel(&mut map, curr_y, prev_y, curr_x);
                        carve_horz_tunnel(&mut map, curr_x, prev_x, prev_y);
                    }
                }

                num_rooms += 1;
                rooms.push(curr_room);
            }
        }
        map
    }
}
