use utils::read_ceres_puzzle_input;

use super::*;

const SEARCHED_WORD: &str = "XMAS";

pub fn ceres_search(input: &str) -> XMASCount {
    let puzzle_matrix = read_ceres_puzzle_input(input);

    let mut match_count: XMASCount = 0;
    // Loop through the chars
    for (line_index, line) in puzzle_matrix.iter().enumerate() {
        for (character_index, character) in line.iter().enumerate() {
            if *character == SEARCHED_WORD.chars().nth(0).unwrap() {
                match_count += check_word_matches_from_start(
                    &puzzle_matrix,
                    character_index,
                    line_index,
                    SEARCHED_WORD,
                );
            }
        }
    }

    match_count
}

fn check_word_matches_from_start(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    word: &str,
) -> XMASCount {
    let mut top_matches: XMASCount = 1;
    let mut bottom_matches: XMASCount = 1;
    let mut left_matches: XMASCount = 1;
    let mut right_matches: XMASCount = 1;

    let mut right_top_matches: XMASCount = 1;
    let mut right_bottom_matches: XMASCount = 1;
    let mut left_top_matches: XMASCount = 1;
    let mut left_bottom_matches: XMASCount = 1;

    for (index, current_char) in word.chars().enumerate() {
        // check to top
        if y as i32 - index as i32 >= 0 {
            if matrix[y - index][x] != current_char {
                top_matches = 0;
            }
        } else {
            top_matches = 0;
        }
        // check to bottom
        if y + index < matrix.len() {
            if matrix[y + index][x] != current_char {
                bottom_matches = 0;
            }
        } else {
            bottom_matches = 0;
        }
        // check left
        if x as i32 - index as i32 >= 0 {
            if matrix[y][x - index] != current_char {
                left_matches = 0;
            }
        } else {
            left_matches = 0;
        }
        // check right
        if x + index < matrix[0].len() {
            if matrix[y][x + index] != current_char {
                right_matches = 0;
            }
        } else {
            right_matches = 0;
        }

        // check left_top
        if x as i32 - index as i32 >= 0 && y as i32 - index as i32 >= 0 {
            if matrix[y - index][x - index] != current_char {
                left_top_matches = 0;
            }
        } else {
            left_top_matches = 0;
        }

        //check right_top
        if x + index < matrix[0].len() && y as i32 - index as i32 >= 0 {
            if matrix[y - index][x + index] != current_char {
                right_top_matches = 0;
            }
        } else {
            right_top_matches = 0;
        }

        //check left_bottom
        if x as i32 - index as i32 >= 0 && y + index < matrix.len() {
            if matrix[y + index][x - index] != current_char {
                left_bottom_matches = 0;
            }
        } else {
            left_bottom_matches = 0;
        }
        //check righ_bottom
        if x + index < matrix[0].len() && y + index < matrix.len() {
            if matrix[y + index][x + index] != current_char {
                right_bottom_matches = 0;
            }
        } else {
            right_bottom_matches = 0;
        }
    }

    right_matches
        + left_matches
        + top_matches
        + bottom_matches
        + right_top_matches
        + left_top_matches
        + right_bottom_matches
        + left_bottom_matches
}

// WRONG: you did not understand the puzzle
// The words must be displayed
// I'll leave it here 4fun
// let mut x_count: XMASCount = 0;
// let mut m_count: XMASCount = 0;
// let mut a_count: XMASCount = 0;
// let mut s_count: XMASCount = 0;
// for letter in puzzle_string.chars() {
//     match letter {
//         'X' => x_count += 1,
//         'M' => m_count += 1,
//         'A' => a_count += 1,
//         'S' => s_count += 1,
//         _ => {}
//     }
// }
// println!("X:{} M:{} A:{} S:{}", x_count, m_count, a_count, s_count);
// if let Some(value) = vec![x_count, m_count, a_count, s_count].iter().min() {
//     return *value;
// }
