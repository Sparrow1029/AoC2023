pub mod grid;
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Lines},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Shared function to get puzzle input as lines
pub fn get_puzzle_input_lines(day: u32) -> Result<Lines<BufReader<File>>> {
    let aoc_home = dotenv::var("AOC_HOME")?;
    let filename = format!("{}/puzzle_inputs/day{:02}.txt", aoc_home, day);
    let file = File::open(filename).expect("input file not found");
    let buf = BufReader::new(file);
    Ok(buf.lines())
}

/// Read puzzle_input to `String`.
pub fn get_puzzle_input_string(day: u32) -> Result<String> {
    let aoc_home = dotenv::var("AOC_HOME")?;
    let filename = format!("{}/puzzle_inputs/day{:02}.txt", aoc_home, day);
    let data = read_to_string(filename)?;
    Ok(data)
}
