use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, digit1, space1},
    sequence::tuple,
    IResult,
};
use rust_aoc2023::{get_puzzle_input_lines, Result};

const DAY_2: u32 = 2;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Record {
    r: u32,
    g: u32,
    b: u32,
}

impl Default for Record {
    fn default() -> Self {
        Record { r: 1, g: 1, b: 1 }
    }
}

#[allow(clippy::double_comparisons)]
impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
    fn le(&self, other: &Self) -> bool {
        (self.r < other.r || self.r == other.r)
            && (self.g < other.g || self.g == other.g)
            && (self.b < other.b || self.b == other.b)
    }
    fn ge(&self, other: &Self) -> bool {
        (self.r > other.r || self.r == other.r)
            && (self.g > other.g || self.g == other.g)
            && (self.b > other.b || self.b == other.b)
    }
    fn lt(&self, other: &Self) -> bool {
        (self.r < other.r) && (self.g < other.g) && (self.b < other.b)
    }
    fn gt(&self, other: &Self) -> bool {
        (self.r > other.r) && (self.g > other.g) && (self.b > other.b)
    }
}

impl Record {
    fn max_vals(&mut self, other: &Record) {
        self.r = std::cmp::max(self.r, other.r);
        self.g = std::cmp::max(self.g, other.g);
        self.b = std::cmp::max(self.b, other.b);
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

fn parse_num_and_color(input: &str) -> IResult<&str, (u32, &str)> {
    let (_, (num, _, color)) = tuple((digit1, space1, alpha0))(input)?;
    let num = num.parse::<u32>().expect("error parsing digit");
    Ok(("", (num, color)))
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let mut r = 0u32;
        let mut g = 0u32;
        let mut b = 0u32;
        for val in value.split(", ") {
            if let Ok((_, (num, color))) = parse_num_and_color(val) {
                match color {
                    "red" => r = num,
                    "blue" => b = num,
                    "green" => g = num,
                    _ => panic!("Unexpected color: {:?}", color),
                }
            }
        }
        Record { r, b, g }
    }
}

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<Record>)> {
    let (rest, (_, game_id, _)) = tuple((tag("Game "), digit1, tag(": ")))(input)?;
    let records = rest.split("; ").map(|s| s.into()).collect();
    Ok((
        rest,
        (
            game_id.parse::<u32>().expect("error parsing game id"),
            records,
        ),
    ))
}

fn part1() -> Result<()> {
    const CMP_RECORD: Record = Record {
        r: 12,
        g: 13,
        b: 14,
    };
    let answer = get_puzzle_input_lines(DAY_2)?
        .filter_map(|l| {
            let (_, (game_id, records)) = parse_line(&l.unwrap()).expect("error parsing line");
            match records.iter().all(|r| r <= &CMP_RECORD) {
                true => Some(game_id),
                false => None,
            }
        })
        .sum::<u32>();
    println!("Part 1: {answer}");
    Ok(())
}

fn part2() -> Result<()> {
    let answer = get_puzzle_input_lines(DAY_2)?
        .map(|l| {
            let (_, (_, records)) = parse_line(&l.unwrap()).expect("error parsing line");
            records
                .iter()
                .fold(Record::default(), |mut orig, rec| {
                    orig.max_vals(rec);
                    orig
                })
                .power()
        })
        .sum::<u32>();
    println!("Part 2: {answer}");
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

#[test]
fn test_aoc_part1() {
    const CMP_RECORD: Record = Record {
        r: 12,
        g: 13,
        b: 14,
    };
    let results: Vec<u32> = TEST_INPUT
        .lines()
        .filter_map(|l| {
            println!("{l}");
            let (_, (game_id, records)) = parse_line(l).expect("error parsing line");
            match records.iter().all(|r| r <= &CMP_RECORD) {
                true => Some(game_id),
                false => None,
            }
        })
        .collect();
    println!("Results: {results:?}");
    println!("Answer: {}", results.iter().sum::<u32>());
    assert_eq!(results.iter().sum::<u32>(), 8)
}

#[test]
fn test_aoc_part2() {
    let result = TEST_INPUT
        .lines()
        .map(|l| {
            let (_, (_, records)) = parse_line(l).expect("error parsing line");
            for r in &records {
                println!("\t{r:?}");
            }
            let final_rec = records.iter().fold(Record::default(), |mut orig, rec| {
                orig.max_vals(rec);
                orig
            });
            println!("  Final: {final_rec:?}");
            println!("  Power: {}", final_rec.power());
            final_rec.power()
        })
        .sum::<u32>();
    println!("Result: {result}");
}
