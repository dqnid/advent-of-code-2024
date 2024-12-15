// mod sun_01;
// mod mon_02;
// mod tue_03;
// mod wed_04;
// mod thu_05;
// mod fri_06;
// mod sat_07;
// mod sun_08;
// mod mon_09;
// mod tue_10;
// mod wed_11;
mod thu_12;
mod types;
mod utils;

use types::*;

pub fn today() {
    historian_hysteria()
}

pub fn historian_hysteria() {
    // NOTE: previous days are commented to speed up (all the blame is on day 6)

    // Sunday 01
    // let key = sun_01::get_key("./assets/day_1_input").unwrap();
    // println!("The key is: {key}");
    // let similarity = sun_01::get_similarity("./assets/day_1_input");
    // println!("The similarity is: {similarity}");

    // Monday 02
    // let safe_report_count = mon_02::check_reports_safety("./assets/day_2_reports_input");
    // println!("There are {safe_report_count} safe reports");

    // Tuesday 03
    // let multiplication_added_result = tue_03::mull_it_over("./assets/day_3_instruction_input");
    // println!(
    //     "The result of the sum of multiplications is {}",
    //     multiplication_added_result
    // );
    // let conditional_multiplication_added_result =
    //     tue_03::mull_it_over_conditional("./assets/day_3_instruction_input");
    // println!(
    //     "The result of the conditional sum of multiplications is {}",
    //     conditional_multiplication_added_result
    // );

    // Wednesday 04
    // let (xmas_appearances, x_mas_appearances) = wed_04::ceres_search("./assets/day_4_input");
    // println!("XMAS appears {} times", xmas_appearances);
    // println!("X-MAS appears {} times", x_mas_appearances);

    // Thursday 05
    // let (queue_mid_sum, fixed_queue_mid_sum) =
    //     thu_05::mid_queue_sum("./assets/day_5_rules_queue_input");
    // println!("The update mid-queue-sum is {}", queue_mid_sum);
    // println!("The fixed update mid-queue-sum is {}", fixed_queue_mid_sum);

    // Friday 06
    // let (guard_position_count, loop_obstacle_count) =
    //     fri_06::guard_gallivant("./assets/day_6_guard_map_input");
    // println!("The guard will visit {} positions", guard_position_count);
    // println!("The guard would loop on {} positions", loop_obstacle_count);

    // Saturday 07
    // let calibration_result = sat_07::bridge_repair("./assets/day_7_calibrations_input");
    // println!("The total calibration result is {}", calibration_result);

    // Sunday 08
    // let (antinode_count, antinode_count_any) =
    //     sun_08::resonant_collinearity("./assets/day_8_antena_map_input");
    // println!("The total antinode positions is {}", antinode_count);
    // println!(
    //     "The total antinode positions in any grid is {}",
    //     antinode_count_any
    // );

    // Monday 09
    // let disk_checksum = mon_09::disk_fragmenter("./assets/day_9_disk_input");
    // println!("The disk checksum is {}", disk_checksum);

    // Tuesday 10
    // let (trailhead_score, full_trailhead_score) =
    //     tue_10::hoof_it("./assets/day_10_trail_map_input");
    // println!("The trail head sum score is {}", trailhead_score);
    // println!("The full trail head sum score is {}", full_trailhead_score);
    // let blink_count: usize = 25;
    // let stone_count =
    //     wed_11::plutonian_pebbles("./assets/day_11_stone_arrangement_input", blink_count);
    // println!(
    //     "The total of stones after {} is {}",
    //     blink_count, stone_count
    // );
    // let blink_count: usize = 75;
    // let stone_count =
    //     wed_11::plutonian_pebbles("./assets/day_11_stone_arrangement_input", blink_count);
    // println!(
    //     "The total of stones after {} is {}",
    //     blink_count, stone_count
    // );
}
