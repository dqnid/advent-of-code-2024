use std::collections::HashSet;

use utils::read_antena_map_input;

use super::*;

pub fn resonant_collinearity(input: &str) -> (AntinodeCount, AntinodeCount) {
    let (roof_map, antena_list) = read_antena_map_input(input);

    (
        get_antinode_set_of_aligned_position(&antena_list, &roof_map).len(),
        get_antinode_set_of_any_aligned_position(&antena_list, &roof_map).len(),
    )
}

pub fn get_antinode_set_of_any_aligned_position(
    antena_list: &AntenaList,
    map: &RoofMap,
) -> AntinodeList {
    let mut antinode_list: AntinodeList = HashSet::new();

    for antena in antena_list {
        for complementary_antena in antena_list {
            if antena != complementary_antena && antena.2 == complementary_antena.2 {
                let distance_x = (antena.1 as i32 - complementary_antena.1 as i32).abs() as usize;
                let distance_y = (antena.0 as i32 - complementary_antena.0 as i32).abs() as usize;

                let mut count: usize = 0;

                // loop {
                //     let test_x = distance_x * count;
                // }

                // antinode_list.insert(pair);
            }
        }
    }

    antinode_list
}

pub fn get_antinode_set_of_aligned_position(
    antena_list: &AntenaList,
    map: &RoofMap,
) -> AntinodeList {
    let mut antinode_list: AntinodeList = HashSet::new();

    for antena in antena_list {
        for complementary_antena in antena_list {
            if antena != complementary_antena && antena.2 == complementary_antena.2 {
                let distance_x = (antena.1 as i32 - complementary_antena.1 as i32).abs() as usize;
                let distance_y = (antena.0 as i32 - complementary_antena.0 as i32).abs() as usize;

                let (a, b) = get_extremes_with_variance(
                    (antena.0, antena.1),
                    (complementary_antena.0, complementary_antena.1),
                    distance_y,
                    distance_x,
                    map.len(),
                    map[0].len(),
                    map,
                );

                if let Some(pair) = a {
                    antinode_list.insert(pair);
                }
                if let Some(pair) = b {
                    antinode_list.insert(pair);
                }
            }
        }
    }

    antinode_list
}

// FIXME: too many if / else, this is easier treating it as a real vector
pub fn get_extremes_with_variance(
    a: (usize, usize),
    b: (usize, usize),
    variance_y: usize,
    variance_x: usize,
    max_y: usize,
    max_x: usize,
    map: &RoofMap,
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    if a.1 > b.1 {
        if a.0 > b.0 {
            // |b_|
            // |_a|
            let first =
                if b.0 as i32 - variance_y as i32 >= 0 && b.1 as i32 - variance_x as i32 >= 0 {
                    let new = (b.0 - variance_y, b.1 - variance_x);
                    if !is_station_between(new, b, map) {
                        Some(new)
                    } else {
                        None
                    }
                } else {
                    None
                };
            let second = if a.0 + variance_y < max_y && a.1 + variance_x < max_x {
                let new = (a.0 + variance_y, a.1 + variance_x);
                if !is_station_between(new, a, &map) {
                    Some(new)
                } else {
                    None
                }
            } else {
                None
            };

            (first, second)
        } else {
            // |_a|
            // |b_|
            let first = if a.0 as i32 - variance_y as i32 >= 0 && a.1 + variance_x < max_x {
                let new = (a.0 - variance_y, a.1 + variance_x);
                if !is_station_between(new, a, &map) {
                    Some(new)
                } else {
                    None
                }
            } else {
                None
            };
            let second = if b.0 + variance_y < max_y && b.1 as i32 - variance_x as i32 >= 0 {
                let new = (b.0 + variance_y, b.1 - variance_x);
                if !is_station_between(new, b, &map) {
                    Some(new)
                } else {
                    None
                }
            } else {
                None
            };

            (first, second)
        }
    } else {
        if a.0 > b.0 {
            // |_b|
            // |a_|
            let first = if b.0 as i32 - variance_y as i32 >= 0 && b.1 + variance_x < max_x {
                let new = (b.0 - variance_y, b.1 + variance_x);
                if !is_station_between(new, b, &map) {
                    Some(new)
                } else {
                    None
                }
            } else {
                None
            };
            let second = if a.0 + variance_y < max_y && a.1 as i32 - variance_x as i32 >= 0 {
                let new = (a.0 + variance_y, a.1 - variance_x);
                if !is_station_between(new, a, &map) {
                    Some(new)
                } else {
                    None
                }
            } else {
                None
            };

            (first, second)
        } else {
            // |a_|
            // |_b|
            let first =
                if a.0 as i32 - variance_y as i32 >= 0 && a.1 as i32 - variance_x as i32 >= 0 {
                    let new = (a.0 - variance_y, a.1 - variance_x);
                    if !is_station_between(new, a, &map) {
                        Some(new)
                    } else {
                        None
                    }
                } else {
                    None
                };
            let second = if b.0 + variance_y < max_y && b.1 + variance_x < max_x {
                let new = (b.0 + variance_y, b.1 + variance_x);
                if !is_station_between(new, b, &map) {
                    Some(new)
                } else {
                    None
                }
            } else {
                None
            };

            (first, second)
        }
    }
}

pub fn is_station_between(a: (usize, usize), b: (usize, usize), map: &RoofMap) -> bool {
    let mut count: usize = 1;

    let y_diff = (a.0 as i32 - b.0 as i32).abs() as usize;
    let x_diff = (a.1 as i32 - b.1 as i32).abs() as usize;

    'check_diagonal: loop {
        let mut next_x: i32 = 0;
        let mut next_y: i32 = 0;

        if a.1 > b.1 {
            if a.0 > b.0 {
                // |b_|
                // |_a|
                next_y = b.0 as i32 + (y_diff * count) as i32;
                next_x = b.1 as i32 + (x_diff * count) as i32;
                if next_x >= a.1 as i32 || next_y >= a.0 as i32 {
                    break 'check_diagonal;
                }
            } else {
                // |_a|
                // |b_|
                next_y = a.0 as i32 + (y_diff * count) as i32;
                next_x = a.1 as i32 - (x_diff * count) as i32;
                if next_x <= b.1 as i32 || next_y >= b.0 as i32 {
                    break 'check_diagonal;
                }
            }
        } else {
            if a.0 > b.0 {
                // |_b|
                // |a_|
                next_y = b.0 as i32 + (x_diff * count) as i32;
                next_x = b.1 as i32 - (y_diff * count) as i32;
                if next_x <= a.1 as i32 || next_y >= b.0 as i32 {
                    break 'check_diagonal;
                }
            } else {
                // |a_|
                // |_b|
                next_y = a.0 as i32 - (x_diff * count) as i32;
                next_x = a.1 as i32 + (y_diff * count) as i32;
                if next_x >= b.1 as i32 || next_y <= b.0 as i32 {
                    break 'check_diagonal;
                }
            }
        }
        if let Some(_) = map[next_y as usize][next_x as usize] {
            return true;
        }

        count += 1;
    }

    false
}

// NOTE: this might be the ugliest piece of code ever
// pub fn get_antinode_set_of_position(
//     x: usize,
//     y: usize,
//     freq: &char,
//     map: &RoofMap,
// ) -> AntinodeList {
//     println!("Checking {},{}", y, x);
//     let mut antinode_list: AntinodeList = HashSet::new();

//     let mut scroll_count: i32 = 1;
//     'all_directions_check: loop {
//         let new_pos_x = x as i32 + scroll_count;
//         let new_pos_y = y as i32 + scroll_count;
//         let new_neg_x = x as i32 - scroll_count;
//         let new_neg_y = y as i32 - scroll_count;

//         if new_pos_x >= map[0].len() as i32
//             && new_pos_y >= map.len() as i32
//             && new_neg_x < 0
//             && new_neg_y < 0
//         {
//             break 'all_directions_check;
//         }

//         // bottom right
//         if new_pos_x < map[0].len() as i32 && new_pos_y < map.len() as i32 {
//             if let Some(map_freq) = map[new_pos_y as usize][new_pos_x as usize] {
//                 println!("Map_freq: {}", map_freq);
//                 if map_freq == *freq {
//                     let distance_between_antenas = (x as i32 - new_pos_x).abs();
//                     let antinode_x = new_pos_x + distance_between_antenas;
//                     let antinode_y = new_pos_y + distance_between_antenas;

//                     if antinode_x < map[0].len() as i32 && antinode_y < map.len() as i32 {
//                         if let None = map[antinode_y as usize][antinode_x as usize] {
//                             antinode_list.insert((antinode_y as usize, antinode_x as usize));
//                         }
//                     }
//                 }
//             }
//         }

//         if new_pos_x < map[0].len() as i32 && new_neg_y >= 0 {
//             if let Some(map_freq) = map[new_neg_y as usize][new_pos_x as usize] {
//                 println!("Map_freq: {}", map_freq);
//                 if map_freq == *freq {
//                     let distance_between_antenas = (x as i32 - new_pos_x).abs();
//                     let antinode_x = new_pos_x + distance_between_antenas;
//                     let antinode_y = new_neg_y - distance_between_antenas;

//                     if antinode_x < map[0].len() as i32 && antinode_y > 0 {
//                         if let None = map[antinode_y as usize][antinode_x as usize] {
//                             antinode_list.insert((antinode_y as usize, antinode_x as usize));
//                         }
//                     }
//                 }
//             }
//         }

//         if new_neg_x >= 0 && new_neg_y >= 0 {
//             if let Some(map_freq) = map[new_neg_y as usize][new_neg_x as usize] {
//                 println!("Map_freq: {}", map_freq);
//                 if map_freq == *freq {
//                     let distance_between_antenas = (x as i32 - new_neg_x).abs();
//                     let antinode_x = new_neg_x - distance_between_antenas;
//                     let antinode_y = new_neg_y - distance_between_antenas;

//                     if antinode_x >= 0 && antinode_y >= 0 {
//                         if let None = map[antinode_y as usize][antinode_x as usize] {
//                             antinode_list.insert((antinode_y as usize, antinode_x as usize));
//                         }
//                     }
//                 }
//             }
//         }

//         if new_neg_x >= 0 && new_pos_y < map.len() as i32 {
//             if let Some(map_freq) = map[new_pos_y as usize][new_neg_x as usize] {
//                 println!("Map_freq: {}", map_freq);
//                 if map_freq == *freq {
//                     let distance_between_antenas = (x as i32 - new_neg_x).abs();
//                     let antinode_x = new_neg_x - distance_between_antenas;
//                     let antinode_y = new_pos_y + distance_between_antenas;

//                     if antinode_x >= 0 && antinode_y < map.len() as i32 {
//                         if let None = map[antinode_y as usize][antinode_x as usize] {
//                             antinode_list.insert((antinode_y as usize, antinode_x as usize));
//                         }
//                     }
//                 }
//             }
//         }

//         scroll_count += 1;
//     }

//     println!("Antinode_list: {:?}", antinode_list);

//     antinode_list
// }
