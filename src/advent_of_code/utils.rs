use std::fs::read_to_string;
use std::ops::Sub;

use super::*;

pub fn read_id_lists(input: &str) -> (Vec<Id>, Vec<Id>) {
    let mut list_1: Vec<Id> = vec![];
    let mut list_2: Vec<Id> = vec![];

    for line in read_to_string(input).unwrap().lines() {
        if let Some((one, two)) = line.split_once("   ") {
            list_1.push(one.parse::<Id>().unwrap());
            list_2.push(two.parse::<Id>().unwrap());
        }
    }

    return (list_1, list_2);
}

pub fn read_report_list(input: &str) -> Vec<Report> {
    let mut report_list: Vec<Report> = vec![];

    for report in read_to_string(input).unwrap().lines() {
        let level_list = report.split(" ");
        let mut report_vec: Report = vec![];
        for level in level_list {
            report_vec.push(level.parse::<Level>().unwrap());
        }
        report_list.push(report_vec);
    }

    report_list
}

pub fn read_instruction_input(input: &str) -> String {
    let mut instructions = String::new();

    for line in read_to_string(input).unwrap().lines() {
        instructions.push_str(line);
    }

    instructions
}

pub fn read_ceres_puzzle_input(input: &str) -> Vec<Vec<char>> {
    let mut puzzle_input: Vec<Vec<char>> = vec![];

    for line in read_to_string(input).unwrap().lines() {
        let mut line_vec: Vec<char> = vec![];
        for character in line.chars().into_iter() {
            line_vec.push(character);
        }
        puzzle_input.push(line_vec);
    }

    puzzle_input
}

pub fn read_rules_and_queue(input: &str) -> (Vec<Rule>, Vec<Queue>) {
    let mut rules: Vec<Rule> = vec![];
    let mut queues: Vec<Queue> = vec![];

    let input_string = read_to_string(input).unwrap();
    if let Some((rule_set, queue_list)) = input_string.split_once("\n\n") {
        // process rules
        for rule in rule_set.split("\n") {
            if let Some((first, second)) = rule.split_once("|") {
                rules.push(Rule(
                    first.parse::<u32>().unwrap(),
                    second.parse::<u32>().unwrap(),
                ));
            }
        }
        // process queues
        for queue in queue_list.split("\n") {
            if queue.len() > 0 {
                let mut queue_vec: Queue = vec![];
                for element in queue.split(",") {
                    queue_vec.push(element.parse::<u32>().unwrap());
                }
                queues.push(queue_vec);
            }
        }
    }

    (rules, queues)
}

pub fn read_floor_map_and_guard_input(input: &str) -> (Guard, FloorMap) {
    let mut floor_map: FloorMap = vec![];
    let mut the_guard: Guard = Guard {
        x: 0,
        y: 0,
        dir: Direction::N,
    };

    for (line_index, line) in read_to_string(input).unwrap().lines().enumerate() {
        let mut line_vec: Vec<Floor> = vec![];
        for (character_index, character) in line.chars().into_iter().enumerate() {
            match character {
                '.' => line_vec.push(Floor::Clear),
                '#' => line_vec.push(Floor::Obstacle),
                '>' | '<' | '^' | 'v' => {
                    let guard_direction: Direction = {
                        match character {
                            '>' => Direction::E,
                            '<' => Direction::W,
                            '^' => Direction::N,
                            'v' => Direction::S,
                            _ => Direction::N,
                        }
                    };
                    the_guard.dir = guard_direction;
                    the_guard.x = character_index;
                    the_guard.y = line_index;
                    line_vec.push(Floor::Clear);
                }
                _ => line_vec.push(Floor::Obstacle),
            }
        }

        floor_map.push(line_vec);
    }

    (the_guard, floor_map)
}

pub fn read_calibration_equations(input: &str) -> Vec<Calibration> {
    let mut calibration_list: Vec<Calibration> = vec![];

    for line in read_to_string(input).unwrap().lines() {
        if let Some((test_value, equation_values)) = line.split_once(": ") {
            let mut values: CalibrationEquation = vec![];
            for equation_value in equation_values.split(" ") {
                values.push(equation_value.parse::<CalibrationResult>().unwrap());
            }
            calibration_list.push(Calibration(
                test_value.parse::<CalibrationResult>().unwrap(),
                values,
            ));
        }
    }

    calibration_list
}

pub fn read_antena_map_input(input: &str) -> (RoofMap, AntenaList) {
    let mut roof_map: RoofMap = vec![];
    let mut antena_list: AntenaList = vec![];

    for (line_index, line) in read_to_string(input).unwrap().lines().enumerate() {
        let mut line_vec: RoofRow = vec![];
        for (character_index, character) in line.chars().into_iter().enumerate() {
            if character == '.' {
                line_vec.push(None);
            } else {
                line_vec.push(Some(character));
                antena_list.push(Antena(line_index, character_index, character));
            }
        }
        roof_map.push(line_vec);
    }

    (roof_map, antena_list)
}

pub fn read_disk_map_input(input: &str) -> DiskMap {
    let mut disk_map: DiskMap = vec![];
    let raw_map = read_to_string(input).unwrap();

    let mut id: usize = 0;
    for (digit_index, char_digit) in raw_map.chars().enumerate() {
        if let Some(digit) = char_digit.to_digit(10u32) {
            for _ in 0..digit {
                if digit_index % 2 == 0 {
                    disk_map.push(Some(id));
                } else {
                    disk_map.push(None);
                }
            }
            if digit_index % 2 == 0 {
                id += 1;
            }
        }
    }

    disk_map
}

pub fn calc_distance<T>(num_1: T, num_2: T) -> T
where
    T: PartialOrd + Sub<Output = T>,
{
    if num_1 > num_2 {
        return num_1 - num_2;
    }
    return num_2 - num_1;
}
