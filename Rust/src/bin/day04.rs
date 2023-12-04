use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use rust_aoc2023::{get_puzzle_input_lines, Result};

const DAY04: u32 = 4;

fn num_list_parser(s: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, u32)(s)
}

fn card_id_parser(s: &str) -> IResult<&str, u32> {
    let (rest, (_, _, card_id)) = tuple((alpha1, multispace1, u32))(s)?;
    Ok((rest, card_id))
}

fn line_parser(s: &str) -> (u32, u32) {
    let (_, (card_id, _, _, winning_nums, _, _, _, card_nums)) = tuple((
        card_id_parser,
        tag(":"),
        multispace1,
        num_list_parser,
        multispace1,
        tag("|"),
        multispace1,
        num_list_parser,
    ))(s)
    .unwrap();
    let winning_nums: HashSet<u32> = HashSet::from_iter(winning_nums);
    let card_nums: HashSet<u32> = HashSet::from_iter(card_nums);
    (
        card_id,
        winning_nums.intersection(&card_nums).count() as u32,
    )
}

fn part_1() -> Result<u32> {
    Ok(get_puzzle_input_lines(DAY04)?
        .map(|l| {
            let (_, score) = line_parser(&l.expect("error reading line"));
            2f32.powi(score as i32 - 1) as u32
        })
        .sum())
}

fn part_2() -> Result<u32> {
    let scores: HashMap<u32, u32> = HashMap::from_iter(
        get_puzzle_input_lines(DAY04)?.map(|l| line_parser(&l.expect("error reading line"))),
    );
    let mut card_counts = vec![1; scores.len()];
    for i in 0..scores.len() {
        for j in i + 1..(i as u32 + scores[&(i as u32 + 1)] + 1) as usize {
            card_counts[j] += card_counts[i]
        }
    }
    Ok(card_counts.iter().sum())
}

fn main() -> Result<()> {
    println!("Part 1: {}", part_1()?);
    println!("Part 2: {}", part_2()?);
    Ok(())
}
