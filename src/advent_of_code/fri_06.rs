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

    fn turn_90(&mut self) {
        match self.dir {
            Direction::N => self.dir = Direction::E,
            Direction::S => self.dir = Direction::W,
            Direction::E => self.dir = Direction::S,
            Direction::W => self.dir = Direction::N,
        }
    }
}

pub fn guard_gallivant(input: &str) -> PositionCount {
    let (mut the_guard, floor_map) = utils::read_floor_map_and_guard_input(input);

    let mut visited_coordinates: HashSet<(usize, usize)> = HashSet::new();

    // FIXME: this logic is not good enough, since the guard could walk on the same place twice, the second time should not be counted
    visited_coordinates.insert((the_guard.x, the_guard.y));
    while !the_guard.step(&floor_map) {
        visited_coordinates.insert((the_guard.x, the_guard.y));
    }

    visited_coordinates.len()
}
