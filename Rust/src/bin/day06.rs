use itertools::join;
use rust_aoc2023::{get_puzzle_input_lines, get_puzzle_input_string};

const DAY_06: u32 = 6;

fn u64_vec_from_str(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|d| d.parse::<u64>().ok())
        .collect()
}

fn parse_input(input: String) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let times = u64_vec_from_str(lines.next().unwrap());
    let distances = u64_vec_from_str(lines.next().unwrap());
    times
        .iter()
        .copied()
        .zip(distances.iter().copied())
        .collect()
}

fn join_u64(input: &str) -> u64 {
    join(input.split_ascii_whitespace().skip(1), "")
        .parse()
        .unwrap()
}

fn calc_possible_wins(time: u64, distance: u64) -> Option<u64> {
    // Second half is a mirror of the first half, only calculate first part
    for hold_time in 1..=time / 2 {
        // If the hold_time crosses the threshold, the following ones will too
        if hold_time * (time - hold_time) > distance {
            // There are time + 1 options (include 0)
            // Subtract twice the minimum required hold time, which also removes the longest
            // push times
            return Some(time + 1 - 2 * hold_time);
        }
    }
    None
}

fn part1(data: &[(u64, u64)]) -> u64 {
    data.iter()
        .filter_map(|(time, distance)| calc_possible_wins(*time, *distance))
        .product()
}

fn part2() -> u64 {
    let input = get_puzzle_input_string(DAY_06).expect("error parsing input");
    let time = join_u64(input.lines().next().unwrap());
    let distance = join_u64(input.lines().nth(1).unwrap());
    calc_possible_wins(time, distance).unwrap()
}

fn main() {
    let data = parse_input(get_puzzle_input_string(DAY_06).expect("error parsing input"));
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2());
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";
#[test]
fn test_part_1() {
    let data = parse_input(TEST_INPUT.to_string());
    assert_eq!(part1(&data), 288);
}

#[test]
fn test_join_u64() {
    let num = join_u64(TEST_INPUT.lines().next().unwrap());
    assert_eq!(num, 71530);
}
