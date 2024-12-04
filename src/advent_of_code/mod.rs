mod mon_02;
mod sun_01;
mod tue_03;
mod types;
mod utils;

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
}
