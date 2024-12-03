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

pub fn read_report_list(input: &str) {}

pub fn calc_distance<T>(num_1: T, num_2: T) -> T
where
    T: PartialOrd + Sub<Output = T>,
{
    if num_1 > num_2 {
        return num_1 - num_2;
    }
    return num_2 - num_1;
}
