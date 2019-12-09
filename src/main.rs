pub mod util;
pub mod days;

use {
    util::{
        get_input,
    },
    days::{
        Day,
        day1
    },
    std::collections::HashMap,
    std::env,
};

const INPUT_BASE_PATH: &str = "/Users/pierre/projects/private/trams/aoc-2019/inputs";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let map = populate_map();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        x if x < 2 => {
            println!("No arguments");
            return Ok(());
        },
        x if x > 2 => {
            println!("To many arguments");
            return Ok(());
        }
        _ => {}
    }

    let day = match args.get(1) {
        Some(arg) => {
            match arg.trim().parse::<u8>() {
                Ok(num) if num > 0 && num < args.len() as u8 => {
                    num
                },
                Ok(x) => {
                    println!("Day not implemented: {}", x);
                    return Ok(())
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return Ok(())
                }
            }
        },
        None => {
            println!("Missing argument");
            return Ok(())
        }
    };

    let input_1 = get_input(&format!("{}/{}", INPUT_BASE_PATH, format!("day{}-1", day)))?;
    let input_2 = get_input(&format!("{}/{}", INPUT_BASE_PATH, format!("day{}-2", day)))?;
    match map.get(&day) {
        Some(day) => {
            println!("First: {}\nSecond: {}", day.first(input_1), day.second(input_2));
            Ok(())
        },
        _ => {
            println!("Day {} not implemented", day);
            Ok(())
        }
    }
}

fn populate_map() -> HashMap<u8, Box<dyn Day>> {
    let mut days: HashMap<u8, Box<dyn Day>> = HashMap::new();
    days.insert(1, Box::new(day1::Day1));
    days
}

