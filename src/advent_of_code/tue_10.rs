use utils::read_trail_map_input;

use super::*;

pub fn hoof_it(input: &str) -> usize {
    let trail_map = read_trail_map_input(input);

    let vector_of_scores = explore_map(trail_map);

    let mut sum_of_scores: usize = 0;

    println!("The score list is: {:?}", vector_of_scores);
    // TODO: improve, this is dirty
    for score in vector_of_scores {
        sum_of_scores += score;
    }

    sum_of_scores
}

pub fn explore_map(map: TrailMap) -> Vec<TrailScore> {
    let mut trail_score_list: Vec<TrailScore> = vec![];

    for y_index in 0..map.len() {
        for x_index in 0..map[y_index].len() {
            if map[y_index][x_index] == 0 {
                trail_score_list.push(explore_position(x_index, y_index, &map));
            }
        }
    }

    trail_score_list
}

pub fn explore_position(x: TrailPosition, y: TrailPosition, map: &TrailMap) -> usize {
    if map[y][x] == 9 {
        return 1;
    }

    let mut score_sum: usize = 0;
    for next_position in get_pos_surroundings(x, y, map) {
        if map[next_position.1][next_position.0] == map[y][x] + 1 {
            score_sum += explore_position(next_position.0, next_position.1, map);
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

        if y > 0 {
            trail_pos_vec.push((x - 1, y - 1));
        }
        if y < map.len() - 1 {
            trail_pos_vec.push((x - 1, y + 1));
        }
    }

    if x < map[0].len() - 1 {
        trail_pos_vec.push((x + 1, y));

        if y > 0 {
            trail_pos_vec.push((x + 1, y - 1));
        }

        if y < map.len() - 1 {
            trail_pos_vec.push((x + 1, y + 1));
        }
    }

    if y > 0 {
        trail_pos_vec.push((x, y - 1));
    }

    if y < map.len() - 1 {
        trail_pos_vec.push((x, y + 1));
    }

    trail_pos_vec
}
