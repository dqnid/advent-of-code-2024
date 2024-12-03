mod sun_01;
mod types;
mod utils;

use types::*;

pub fn today() {
    historian_hysteria()
}

pub fn historian_hysteria() {
    let key = sun_01::sun_01("./assets/day_1_input").unwrap();
    println!("The key is: {key}");
    let similarity = sun_01::get_similarity("./assets/day_1_input");
    println!("The similarity is: {similarity}");
}
