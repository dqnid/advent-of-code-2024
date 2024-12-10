use primes::{PrimeSet, Sieve};

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
    let operation_variants = generate_operation_variants(equation.1.len() as u32 - 1);

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

// FIXME: this goes wrong on large numbers
pub fn generate_operation_variants(count: u32) -> Vec<Vec<Operation>> {
    let mut variant_list: Vec<Vec<Operation>> = vec![];

    let mut pset = Sieve::new(); // get prime numbers

    let variant_num = 2i32.pow(count);

    for variant_index in 0..variant_num {
        let mut new_vec: Vec<Operation> = vec![];
        for new_operand_index in 0..count {
            match variant_index as u64 % pset.get(new_operand_index as usize) {
                0 => new_vec.push(Operation::MUL),
                _ => new_vec.push(Operation::SUM),
            }
        }
        variant_list.push(new_vec);
    }

    variant_list
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
