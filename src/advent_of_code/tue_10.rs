use utils::read_trail_map_input;

use super::*;

pub fn hoof_it(input: &str) -> (usize, usize) {
    let trail_map = read_trail_map_input(input);

    let (vector_of_scores, vector_of_all_scores) = explore_map(trail_map);

    let mut sum_of_scores: usize = 0;

    // TODO: improve, this is dirty
    for score in vector_of_scores {
        sum_of_scores += score;
    }

    let mut sum_of_all_scores: usize = 0;
    for score in vector_of_all_scores {
        sum_of_all_scores += score;
    }

    (sum_of_scores, sum_of_all_scores)
}

pub fn explore_map(map: TrailMap) -> (Vec<TrailScore>, Vec<TrailScore>) {
    let mut trail_score_list: Vec<TrailScore> = vec![];
    let mut full_trail_score_list: Vec<TrailScore> = vec![];

    for y_index in 0..map.len() {
        for x_index in 0..map[y_index].len() {
            if map[y_index][x_index] == 0 {
                let mut reached_ends: Vec<(usize, usize)> = vec![];
                trail_score_list.push(explore_position(x_index, y_index, &map, &mut reached_ends));
                full_trail_score_list.push(explore_all_positions(x_index, y_index, &map));
            }
        }
    }

    (trail_score_list, full_trail_score_list)
}

pub fn explore_position(
    x: TrailPosition,
    y: TrailPosition,
    map: &TrailMap,
    reached_ends: &mut Vec<(usize, usize)>,
) -> usize {
    if map[y][x] == 9 {
        reached_ends.push((x, y));
        return 1;
    }

    let mut score_sum: usize = 0;
    for next_position in get_pos_surroundings(x, y, map) {
        if map[next_position.1][next_position.0] == map[y][x] + 1
            && !reached_ends.contains(&(next_position.0, next_position.1))
        {
            score_sum += explore_position(next_position.0, next_position.1, map, reached_ends);
        }
    }

    score_sum
}

pub fn explore_all_positions(x: TrailPosition, y: TrailPosition, map: &TrailMap) -> usize {
    if map[y][x] == 9 {
        return 1;
    }

    let mut score_sum: usize = 0;
    for next_position in get_pos_surroundings(x, y, map) {
        if map[next_position.1][next_position.0] == map[y][x] + 1 {
            score_sum += explore_all_positions(next_position.0, next_position.1, map);
        }
    }

    score_sum
}

pub fn get_pos_surroundings(
    x: TrailPosition,
    y: TrailPosition,
    map: &TrailMap,
) -> Vec<(usize, usize)> {
    let mut trail_pos_vec: Vec<(usize, usize)> = vec![];

    if x > 0 {
        trail_pos_vec.push((x - 1, y));

        // if y > 0 {
        //     trail_pos_vec.push((x - 1, y - 1));
        // }
        // if y < map.len() - 1 {
        //     trail_pos_vec.push((x - 1, y + 1));
        // }
    }

    if x < map[0].len() - 1 {
        trail_pos_vec.push((x + 1, y));

        // if y > 0 {
        //     trail_pos_vec.push((x + 1, y - 1));
        // }
        //
        // if y < map.len() - 1 {
        //     trail_pos_vec.push((x + 1, y + 1));
        // }
    }

    if y > 0 {
        trail_pos_vec.push((x, y - 1));
    }

    if y < map.len() - 1 {
        trail_pos_vec.push((x, y + 1));
    }

    trail_pos_vec
}
