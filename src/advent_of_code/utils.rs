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

pub fn calc_distance<T>(num_1: T, num_2: T) -> T
where
    T: PartialOrd + Sub<Output = T>,
{
    if num_1 > num_2 {
        return num_1 - num_2;
    }
    return num_2 - num_1;
}
