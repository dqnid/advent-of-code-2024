use super::*;

pub fn plutonian_pebbles(input: &str, blink_count: usize) -> StoneCount {
    let stone_list = utils::read_stone_arrangement(input);

    let stone_list = apply_rules(stone_list, blink_count);

    stone_list.len()
}

fn apply_rules(stone_list: StoneList, blink_count: usize) -> StoneList {
    let mut new_stone_list = stone_list.clone();

    for _ in 0..blink_count {
        new_stone_list = blink(new_stone_list);
    }

    new_stone_list
}

fn blink(stone_list: StoneList) -> StoneList {
    let mut new_stone_list: StoneList = vec![];

    const STONE_MULTIPLIER: usize = 2024;
    for stone in stone_list {
        match stone {
            0 => {
                new_stone_list.push(1);
            }
            stone if stone.to_string().len() % 2 == 0 => {
                let (left, right) = split_num(stone);
                new_stone_list.push(left);
                new_stone_list.push(right);
            }
            _ => {
                new_stone_list.push(stone * STONE_MULTIPLIER);
            }
        }
    }

    new_stone_list
}

fn split_num(num: Stone) -> (Stone, Stone) {
    let split_index = num.to_string().len() / 2;
    let binding = num.to_string();
    let (first, second) = binding.split_at(split_index);

    (
        first.parse::<Stone>().unwrap(),
        second.parse::<Stone>().unwrap(),
    )
}
