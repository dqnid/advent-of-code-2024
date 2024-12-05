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

pub fn mull_it_over_conditional(input: &str) -> MulNumber {
    let instruction_input = read_instruction_input(input);

    let find_condition_and_multiply_regex =
        Regex::new(r"(do\(\)|don't\(\).*?mul\([0-9]+,[0-9]+\)|.*?mul\([0-9]+,[0-9]+\))").unwrap();
    let separate_condition_and_multiply_regex =
        Regex::new(r"(do\(\)|don't\(\)).*?(mul\([0-9]+,[0-9]+\))").unwrap();
    let mul_regex = Regex::new(r"(mul\([0-9]+,[0-9]+\))").unwrap();
    let conditional_regex = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let mut sum_result: MulNumber = 0;
    let mut current_do_condition: bool = true;

    for conditional_mul in find_condition_and_multiply_regex.captures_iter(&instruction_input) {
        if let Some(operation_pair) =
            separate_condition_and_multiply_regex.captures(&conditional_mul[1])
        {
            // conditional AND mul present
            let next_conditional = conditional_regex.captures(&operation_pair[1]);
            if let Some(c) = next_conditional {
                if &c[1] == "do()" {
                    current_do_condition = true;
                } else {
                    current_do_condition = false;
                }
            }
            if let Some(o) = mul_regex.captures(&operation_pair[2]) {
                if current_do_condition {
                    let multiplication_result = process_mul_operation(&o[1]);
                    sum_result += multiplication_result;
                }
            }
        } else if let Some(c) = conditional_regex.captures(&conditional_mul[1]) {
            // only conditional present
            if &c[1] == "do()" {
                current_do_condition = true;
            } else {
                current_do_condition = false;
            }
        } else if let Some(o) = mul_regex.captures(&conditional_mul[1]) {
            // only mul present
            if current_do_condition {
                let multiplication_result = process_mul_operation(&o[1]);
                sum_result += multiplication_result;
            }
        }
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
