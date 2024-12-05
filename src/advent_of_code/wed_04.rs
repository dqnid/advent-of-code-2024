use utils::read_ceres_puzzle_input;

use super::*;

const SEARCHED_WORD: &str = "XMAS";
const X_SEARCHED_WORD: &str = "MAS";

pub fn ceres_search(input: &str) -> (XMASCount, XMASCount) {
    let puzzle_matrix = read_ceres_puzzle_input(input);

    let mut match_count: XMASCount = 0;
    let mut x_match_count: XMASCount = 0;
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

            if *character
                == X_SEARCHED_WORD
                    .chars()
                    .nth(SEARCHED_WORD.len() / 2 - 1)
                    .unwrap()
            {
                x_match_count +=
                    check_x_word_matches_from_center(&puzzle_matrix, character_index, line_index)
            }
        }
    }

    (match_count, x_match_count)
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

// NOTE: V1: only works for MAS word, quite dirty tho...
fn check_x_word_matches_from_center(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> XMASCount {
    if x < 1 || x > matrix[0].len() - 2 || y < 1 || y > matrix.len() - 2 {
        return 0;
    }

    if is_string_mas(format!(
        "{}{}{}",
        matrix[y - 1][x - 1],
        matrix[y][x],
        matrix[y + 1][x + 1]
    )) && is_string_mas(format!(
        "{}{}{}",
        matrix[y - 1][x + 1],
        matrix[y][x],
        matrix[y + 1][x - 1]
    )) {
        return 1;
    }
    0
}

fn is_string_mas(word: String) -> bool {
    word == "MAS" || word == "SAM"
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
