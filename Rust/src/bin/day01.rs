use rust_aoc2023::{get_puzzle_input_lines, Result};

fn main() -> Result<()> {
    println!("Day 01");
    for line in get_puzzle_input_lines(1)? {
        println!("{:?}", line?);
    }
    Ok(())
}
