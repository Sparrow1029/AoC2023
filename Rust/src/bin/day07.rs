use std::cmp::Ordering;

use itertools::Itertools;
use rust_aoc2023::get_puzzle_input_string;

const DAY_07: u32 = 7;

type Cards = [usize; 5];
type Bid = usize;
type Hand = (Cards, Bid);
type Count<'a> = (usize, &'a usize);

// Rank values for readability
const FIVE_OF_A_KIND: usize = 7;
const FOUR_OF_A_KIND: usize = 6;
const FULL_HOUSE: usize = 5;
const THREE_OF_A_KIND: usize = 4;
const TWO_PAIR: usize = 3;
const ONE_PAIR: usize = 2;
const HIGH_CARD: usize = 1;

fn parse_str<const N: usize>(input: &str, joker: bool) -> ([usize; N], usize) {
    let mut cards = [0usize; N];
    let split = input.split_whitespace().collect::<Vec<&str>>();
    split[0].chars().enumerate().for_each(|(i, c)| {
        cards[i] = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if joker {
                    0
                } else {
                    11
                }
            }
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            '1' => 1,
            _ => panic!("Found unknown character"),
        }
    });
    (cards, split[1].parse().unwrap())
}

fn high_card_frequency(x: Count, y: Count) -> Ordering {
    let ((cnt_a, a), (cnt_b, b)) = (x, y);
    if cnt_a == cnt_b {
        return b.cmp(a);
    }
    cnt_b.cmp(&cnt_a)
}

fn get_hand_score(cards: &Cards, joker: bool) -> usize {
    let initial_counts: Vec<Count> = cards
        .iter()
        // don't count jokers until we know their actual card value
        .filter(|c| *c != &0usize)
        .sorted()
        .dedup_with_count()
        // make sure in the case of a tie (two pair), we get the higher card value
        .sorted_by(|a, b| high_card_frequency(*a, *b))
        .collect();
    let mut new_cards: Cards = [0; 5];
    if initial_counts.is_empty() {
        // All jokers, 5 of a kind
        return 7;
    }
    let counts = if joker && cards.contains(&0) {
        // Check for zeroes (jokers)
        // if there are any, get most frequently occuring card
        // replace zeroes with that card, redo the counts
        let most_frequent = initial_counts[0].1;
        cards.iter().enumerate().for_each(|(i, card_val)| {
            new_cards[i] = if *card_val == 0 {
                *most_frequent
            } else {
                *card_val
            };
        });
        new_cards
            .iter()
            .sorted()
            .dedup_with_count()
            .sorted_by(|a, b| high_card_frequency(*a, *b))
            .collect()
    } else {
        initial_counts
    };
    get_rank(&counts)
}

fn get_rank(counts: &[Count]) -> usize {
    let top_card_count = counts[0].0;
    let unique_cards = counts.len();
    match unique_cards {
        1 => FIVE_OF_A_KIND,
        2 => match top_card_count {
            4 => FOUR_OF_A_KIND,
            3 => FULL_HOUSE,
            _ => unreachable!(),
        },
        3 => {
            if top_card_count == 3 {
                THREE_OF_A_KIND
            } else {
                TWO_PAIR
            }
        }
        4 => ONE_PAIR,
        5 => HIGH_CARD,
        _ => unreachable!(),
    }
}

fn solve(input: &str, joker: bool) -> usize {
    let hands = input
        .lines()
        .map(|l| {
            let hand: Hand = parse_str(l, joker);
            hand
        })
        .sorted_by(|a, b| {
            let a_score = get_hand_score(&a.0, joker);
            let b_score = get_hand_score(&b.0, joker);
            if a_score == b_score {
                return a.0.cmp(&b.0);
            }
            a_score.cmp(&b_score)
        });
    hands.enumerate().map(|(i, (_, bid))| bid * (i + 1)).sum()
}

fn main() {
    let input = get_puzzle_input_string(DAY_07).expect("I/O error");
    println!("Part 1: {}", solve(&input, false));
    println!("Part 2: {}", solve(&input, true))
}

#[cfg(test)]
mod test {
    use crate::{get_hand_score, parse_str, solve};

    const SAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_solve_part1_with_sample() {
        let ans = solve(SAMPLE_INPUT, false);
        assert_eq!(ans, 6440);
    }

    #[test]
    fn test_solve_part2_with_sample() {
        let ans = solve(SAMPLE_INPUT, true);
        assert_eq!(ans, 5905);
    }

    #[test]
    fn test_edge_cases() {
        let joker = true;
        let hand1: ([usize; 5], usize) = parse_str("J42JJ 42", joker);
        // should be 4 of a kind
        assert_eq!(6, get_hand_score(&hand1.0, joker));
        // should be 5 of a kind
        let hand2: ([usize; 5], usize) = parse_str("JJJJJ 42", joker);
        assert_eq!(7, get_hand_score(&hand2.0, joker));
    }
}
