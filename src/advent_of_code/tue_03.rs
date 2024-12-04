use regex::Regex;
use utils::read_instruction_input;

use super::*;

pub fn mull_it_over(input: &str) -> MulNumber {
    let instruction_input = read_instruction_input(input);

    // NOTE: this regex could avoid process_mul_operation by capturing the values, but seems more readable this way
    let multiply_regex = Regex::new(r"(mul\([0-9]+,[0-9]+\))").unwrap();

    let mut sum_result: MulNumber = 0;
    for mul in multiply_regex.captures_iter(&instruction_input) {
        let multiplication_result = process_mul_operation(&mul[1]);
        sum_result += multiplication_result;
    }

    sum_result
}

fn process_mul_operation(operation: &str) -> MulNumber {
    let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for values in mul_regex.captures_iter(operation) {
        return &values[1].parse::<MulNumber>().unwrap() * &values[2].parse::<MulNumber>().unwrap();
    }

    0
}
