use std::collections::HashSet;

use super::*;

impl Guard {
    fn step(&mut self, map: &FloorMap) -> IsGuardOut {
        let mut future_x = self.x;
        let mut future_y = self.y;

        match self.dir {
            Direction::N => {
                if self.y == 0 {
                    return true;
                }
                future_y -= 1
            }
            Direction::S => {
                if self.y == map.len() - 1 {
                    return true;
                }
                future_y += 1
            }
            Direction::E => {
                if self.x == map[0].len() - 1 {
                    return true;
                }
                future_x += 1;
            }
            Direction::W => {
                if self.x == 0 {
                    return true;
                }
                future_x -= 1;
            }
        }

        match map[future_y][future_x] {
            Floor::Clear => {
                self.x = future_x;
                self.y = future_y;
            }
            Floor::Obstacle => {
                self.turn_90();
                return self.step(map);
            }
        }

        false
    }

    fn will_loop(&mut self, map: &FloorMap) -> bool {
        let mut prev_positions: HashSet<(usize, usize, usize)> = HashSet::new();

        while !self.step(map) {
            let new_prev_position = (self.x, self.y, self.dir as usize);
            if prev_positions.contains(&new_prev_position) {
                return true;
            }
            prev_positions.insert(new_prev_position);
        }

        false
    }

    fn turn_90(&mut self) {
        match self.dir {
            Direction::N => self.dir = Direction::E,
            Direction::S => self.dir = Direction::W,
            Direction::E => self.dir = Direction::S,
            Direction::W => self.dir = Direction::N,
        }
    }
}

pub fn guard_gallivant(input: &str) -> (PositionCount, usize) {
    let (mut the_guard, floor_map) = utils::read_floor_map_and_guard_input(input);

    let mut visited_coordinates: HashSet<(usize, usize)> = HashSet::new();

    // FIXME: this logic is not good enough, since the guard could walk on the same place twice, the second time should not be counted
    visited_coordinates.insert((the_guard.x, the_guard.y));
    while !the_guard.step(&floor_map) {
        visited_coordinates.insert((the_guard.x, the_guard.y));
    }

    let (the_guard, mut floor_map) = utils::read_floor_map_and_guard_input(input);
    let mut loop_obstacles_count: usize = 0;
    for visited in &visited_coordinates {
        let mut temp_guard = the_guard.clone();
        floor_map[visited.1][visited.0] = Floor::Obstacle;
        if temp_guard.will_loop(&floor_map) {
            loop_obstacles_count += 1;
        }
        floor_map[visited.1][visited.0] = Floor::Clear;
    }

    (visited_coordinates.len(), loop_obstacles_count)
}
