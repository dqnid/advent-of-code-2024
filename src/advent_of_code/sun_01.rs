use super::*;

pub fn get_key(input: &str) -> Result<Key, ()> {
    let mut key: Id = 0;

    let (mut list_1, mut list_2) = utils::read_id_lists(input);

    if list_1.len() != list_2.len() {
        return Err(());
    }

    // 1. Pair up the numbers by size and compare the distance between them: Sorting
    list_1.sort();
    list_2.sort();

    // 2. Calc the distance between the pair elements
    for index in 0..list_1.len() {
        key += utils::calc_distance(list_1[index], list_2[index]);
    }

    Ok(key)
}

pub fn get_similarity(input: &str) -> Similarity {
    let (left, right) = utils::read_id_lists(input);
    let mut similarity: Similarity = 0;

    // Multiply every number in left by the number of times that its present in right
    // Then sum every result
    for left_element in left {
        let mut count = 0;
        for right_elment in &right {
            if left_element == *right_elment {
                count += 1;
            }
        }
        similarity += left_element * count;
    }

    similarity
}
