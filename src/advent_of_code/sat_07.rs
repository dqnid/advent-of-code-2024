use super::*;

pub fn bridge_repair(input: &str) -> CalibrationResult {
    let equation_vec = utils::read_calibration_equations(input);
    let mut sum_result: CalibrationResult = 0;

    for equation in equation_vec {
        if is_equation_true(&equation) {
            sum_result += equation.0;
        }
    }

    sum_result
}

pub fn is_equation_true(equation: &Calibration) -> bool {
    let operation_variants = generate_operation_variants(equation.1.len() - 1);

    for variant in operation_variants {
        let mut local_operands = equation.1.clone();
        local_operands.reverse();
        let result = calc_operation(local_operands, variant);
        if result == equation.0 {
            return true;
        }
    }

    false
}

pub fn generate_operation_variants(count: usize) -> Vec<Vec<Operation>> {
    let variants = vec![Operation::MUL, Operation::SUM];
    let variant_list: Vec<Vec<Operation>> = generate_combinations(&variants, count);

    variant_list
}

fn generate_combinations(values: &[Operation], length: usize) -> Vec<Vec<Operation>> {
    if length == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];

    for &value in values {
        let smaller_combinations = generate_combinations(values, length - 1);
        for mut smaller in smaller_combinations {
            smaller.push(value);
            result.push(smaller);
        }
    }

    result
}

pub fn calc_operation(
    operands: CalibrationEquation,
    operators: Vec<Operation>,
) -> CalibrationResult {
    if operands.len() == 2 {
        return operate(&operands[0], &operands[1], &operators[0]);
    }

    return operate(
        &operands[0],
        &calc_operation(operands[1..].to_vec(), operators[1..].to_vec()),
        &operators[0],
    );
}

pub fn operate(
    first: &CalibrationResult,
    second: &CalibrationResult,
    operation: &Operation,
) -> CalibrationResult {
    match operation {
        Operation::SUM => {
            return first + second;
        }
        Operation::MUL => {
            return first * second;
        }
    }
}

// Old strategy, big fail
// pub fn generate_n_square_free_numbers(n: usize) -> Vec<u128> {
//     let mut list: Vec<u128> = vec![];

//     let mut current_n: u128 = 1234;
//     loop {
//         if list.len() == n {
//             break;
//         }

//         if is_square_free(current_n) {
//             list.push(current_n);
//         }

//         current_n += 1;
//     }

//     return list;
// }

// pub fn is_square_free(number: u128) -> bool {
//     let mut i = 2;
//     while i * i <= number {
//         if number % (i * i) == 0 {
//             return false;
//         }
//         i += 1
//     }
//     return true;
// }
