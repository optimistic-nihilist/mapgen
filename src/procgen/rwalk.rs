use crate::{fundamentals::*, maptools::*, utils::*};

const MAX_WALKERS: i32 = 10;
const MAX_STEPS: i32 = 200;

#[derive(Copy, Clone, Debug)]
struct Walker {
    x: i32,
    y: i32,
    pub steps: i32,
}

impl Walker {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            steps: MAX_STEPS,
        }
    }

    fn step(&mut self, map: &mut Map) {
        if self.steps <= 0 {
            return;
        }

        let direction = randr(0..4);
        let (mut dx, mut dy) = (0, 0);
        match direction {
            0 => dx = 1,
            1 => dx = -1,
            2 => dy = 1,
            _ => dy = -1,
        }

        let tx = self.x + dx;
        let ty = self.y + dy;

        if tx < 1 || tx >= COLS - 1 || ty < 1 || ty >= ROWS - 1 {
            return;
        }

        self.x = tx;
        self.y = ty;
        self.steps -= 1;
        map[self.y as usize][self.x as usize] = TileType::Floor as i32;
    }
}

fn spawn_walker<'a>(x: i32, y: i32, vec: &'a mut Vec<Walker>, map: &mut Map) {
    vec.push(Walker::new(x, y));
    map[y as usize][x as usize] = TileType::Floor as i32;
}

pub struct RandomWalkGenerator {}
impl RandomWalkGenerator {
    pub fn generate_map() -> Map {
        let mut map = new_map(TileType::Wall);
        let mut walkers: Vec<Walker> = Vec::new();
        let mut num_walkers = 0;

        spawn_walker(COLS / 2, ROWS / 2, &mut walkers, &mut map);
        num_walkers += 1;

        // until we have active walkers
        while walkers.len() > 0 {
            // create vector for possible newly spawned walkers in this iteration
            let mut walkers_spawned: Vec<Walker> = Vec::new();

            // each walker takes step
            for w in &mut walkers {
                w.step(&mut map);
                // after each step, chance to spawn new walker at walker's current location (if we can)
                if (num_walkers < MAX_WALKERS) & (randr(0..100) > 80) {
                    spawn_walker(w.x, w.y, &mut walkers_spawned, &mut map);
                    num_walkers += 1;
                }
            }

            // if we did spawn new walker in this iteration, append it to the main walkers vector
            if walkers_spawned.len() > 0 {
                walkers.append(&mut walkers_spawned);
            }

            // keep only walkers that still have steps left in main walkers vector
            walkers.retain(|x| x.steps > 0);
        }

        map
    }
}
