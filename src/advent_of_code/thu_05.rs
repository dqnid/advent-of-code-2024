use utils::read_rules_and_queue;

use super::*;

pub fn mid_queue_sum(input: &str) -> u32 {
    let (rules, queues) = read_rules_and_queue(input);

    let mut valid_queue_list: Vec<Queue> = vec![];

    let mut sum_of_mids: u32 = 0;

    for queue in queues {
        if is_queue_valid(&queue, &rules) {
            valid_queue_list.push(queue);
        }
    }

    for queue in valid_queue_list {
        sum_of_mids += queue[queue.len() / 2];
    }

    sum_of_mids
}

fn is_queue_valid(queue: &Queue, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if let Ok(first_index) = get_index_of(queue, &rule.0) {
            if let Ok(second_index) = get_index_of(queue, &rule.1) {
                if first_index > second_index {
                    return false;
                }
            }
        }
    }
    true
}

fn get_index_of<T>(vec: &Vec<T>, element: &T) -> Result<usize, ()>
where
    T: PartialEq,
{
    for (index, e) in vec.iter().enumerate() {
        if e == element {
            return Ok(index);
        }
    }
    Err(())
}
