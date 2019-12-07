pub mod util;
pub mod days;

use {
    util::{
        get_input,
    },
    days::{
        day1
    }
};

const INPUT_BASE_PATH: &str = "/Users/pierre/projects/private/trams/aoc-2019/inputs";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = get_input(&format!("{}/{}", INPUT_BASE_PATH, "day1-1"))?;
    let day1_res = day1::first(input.clone());
    println!("Day1.1: {}", day1_res);
    let day1_1_res = day1::second(input);
    println!("Day1.1: {}", day1_1_res);
    Ok(())
}
