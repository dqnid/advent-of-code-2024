use std::{fs::read_to_string, ops::Sub};

type Number = i32;
type Key = i32;

pub fn sun_01(input: &str) -> Result<Key, ()> {
    let mut list_1: Vec<Number> = vec![];
    let mut list_2: Vec<Number> = vec![];
    let mut key: Number = 0;

    for line in read_to_string(input).unwrap().lines() {
        if let Some((one, two)) = line.split_once("   ") {
            list_1.push(one.parse::<Number>().unwrap());
            list_2.push(two.parse::<Number>().unwrap());
        }
    }

    if list_1.len() != list_2.len() {
        return Err(());
    }

    // 1. Pair up the numbers by size and compare the distance between them: Sorting
    list_1.sort();
    list_2.sort();

    // 2. Calc the distance between the pair elements
    for index in 0..list_1.len() {
        key += calc_distance(list_1[index], list_2[index]);
    }

    Ok(Key)
}

fn calc_distance<T>(num_1: T, num_2: T) -> T
where
    T: PartialOrd + Sub<Output = T>,
{
    if num_1 > num_2 {
        return num_1 - num_2;
    }
    return num_2 - num_1;
}
