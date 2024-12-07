mod fri_06;
mod mon_02;
mod sun_01;
mod thu_05;
mod tue_03;
mod types;
mod utils;
mod wed_04;

use types::*;

pub fn today() {
    historian_hysteria()
}

pub fn historian_hysteria() {
    // Sunday 01
    let key = sun_01::get_key("./assets/day_1_input").unwrap();
    println!("The key is: {key}");
    let similarity = sun_01::get_similarity("./assets/day_1_input");
    println!("The similarity is: {similarity}");

    // Monday 02
    let safe_report_count = mon_02::check_reports_safety("./assets/day_2_reports_input");
    println!("There are {safe_report_count} safe reports");

    // Tuesday 03
    let multiplication_added_result = tue_03::mull_it_over("./assets/day_3_instruction_input");
    println!(
        "The result of the sum of multiplications is {}",
        multiplication_added_result
    );
    let conditional_multiplication_added_result =
        tue_03::mull_it_over_conditional("./assets/day_3_instruction_input");
    println!(
        "The result of the conditional sum of multiplications is {}",
        conditional_multiplication_added_result
    );

    // Wednesday 04
    let (xmas_appearances, x_mas_appearances) = wed_04::ceres_search("./assets/day_4_input");
    println!("XMAS appears {} times", xmas_appearances);
    println!("X-MAS appears {} times", x_mas_appearances);

    // Thursday 05
    let (queue_mid_sum, fixed_queue_mid_sum) =
        thu_05::mid_queue_sum("./assets/day_5_rules_queue_input");
    println!("The update mid-queue-sum is {}", queue_mid_sum);
    println!("The fixed update mid-queue-sum is {}", fixed_queue_mid_sum);

    // Friday 06
    let guard_position_count = fri_06::guard_gallivant("./assets/day_6_guard_map_input");
    println!("The guard will visit {} positions", guard_position_count);
}
