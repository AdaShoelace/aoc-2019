use {
    std::fs::File,
    std::io::{self, prelude::*, BufReader}
};

pub fn get_input(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let ret = reader
        .lines()
        .map(|line| line.unwrap())
        .map(String::from)
        .collect::<Vec<String>>();
    Ok(ret)
}
