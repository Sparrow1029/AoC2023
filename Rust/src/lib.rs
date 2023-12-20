pub mod grid;
pub mod point;
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

/// Greatest common divisor function
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

/// Least common multiple for array of numbers
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

#[allow(dead_code)]
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
