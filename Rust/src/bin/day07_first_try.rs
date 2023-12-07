use self::{Card::*, Play::*};
use core::cmp::Ordering;
use itertools::Itertools;

use rust_aoc2023::get_puzzle_input_string;

const DAY_07: u32 = 7;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            '1' => One,
            _ => panic!("Found unknown character"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Play {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

// impl Ord for Hand {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.kind().cmp(&other.kind())
//     }
// }
//
// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.kind() == other.kind() {
//             return Some(self.cards.cmp(&other.cards));
//         }
//         Some(self.kind().cmp(&other.kind()))
//     }
// }

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let split = value.split_whitespace().collect::<Vec<&str>>();
        let cards = split[0].chars().map(|c| c.into()).collect();
        let bid = split[1].parse::<usize>().unwrap();
        let hand = Hand { cards, bid };
        // println!("created {hand:?} - {:?}", hand.kind());
        hand
    }
}

impl Hand {
    /// Use `itertools::dedup_with_count` to get vec of cards with their counts
    /// sorted by count ascending
    fn get_counts(&self) -> Vec<(usize, Card)> {
        let ans = self
            .cards
            .clone()
            .into_iter()
            // .sorted()
            .dedup_with_count()
            .sorted_by(|(cnta, _), (cntb, _)| cntb.cmp(cnta))
            .collect();
        // println!("Hand after sort:\n\t{:?}", self.cards);
        ans
    }

    fn kind(&self) -> Play {
        let counts = self.get_counts();
        // println!("{counts:?}");
        match counts.len() {
            1 => FiveOfAKind,
            2 => match counts[0].0 {
                4 => FourOfAKind,
                3 => FullHouse,
                _ => unreachable!(),
            },
            3 => {
                if counts[0].0 == 3 {
                    ThreeOfAKind
                } else {
                    TwoPair
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!(),
        }
    }
}

fn part_1(input: String) -> usize {
    // let mut hands = input.lines().map(Hand::from).collect::<Vec<Hand>>();
    // hands.sort();
    // for hand in hands.iter() {
    //     println!("hand after sort: {:?}", hand.cards);
    // }
    input
        .lines()
        .map(Hand::from)
        .sorted_by_key(|hand| hand.kind())
        .sorted_by_key(|hand| hand.cards.clone())
        .enumerate()
        .map(|(i, hand)| {
            println!("Hand after sort: {:?}", hand.cards);
            hand.bid * (i + 1)
        })
        .sum()
}

fn main() {
    let input = get_puzzle_input_string(DAY_07).expect("I/O Error");
    let part1_answer = part_1(input.clone());
    println!("Part 1: {part1_answer}");
}

#[allow(dead_code)]
const SAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn test_parse() {
    let hands: Vec<Hand> = SAMPLE_INPUT.lines().map(Hand::from).collect();
    for hand in hands.iter() {
        println!("{hand:?} - {:?}", hand.kind());
    }
    assert_eq!(hands[0].cards, vec![Three, Two, Ten, Three, King]);
    assert_eq!(hands[0].bid, 765);
}

#[test]
fn test_card_sort() {
    let mut cards = vec![Ten, Three, Ace, King, Jack];
    cards.sort_by(|a, b| b.cmp(a));
    // println!("cards: {cards:?}")
    assert_eq!(cards, vec![Ace, King, Jack, Ten, Three]);

    let mut to_sort = vec![
        vec![Ace, Queen, Queen, Queen, Jack],
        vec![King, King, Seven, Seven, Six],
    ];
    to_sort.sort();
    assert_eq!(
        to_sort,
        vec![
            vec![King, King, Seven, Seven, Six],
            vec![Ace, Queen, Queen, Queen, Jack],
        ]
    );
}

#[test]
fn test_counts() {
    let mut hands: Vec<Hand> = SAMPLE_INPUT.lines().map(Hand::from).collect();
    hands.sort();
    // for hand in hands.iter() {
    //     println!("{hand:?} - {:?}", hand.kind());
    // }
    #[rustfmt::skip]
    assert_eq!(hands, vec![
        Hand { cards: vec![King, Ten, Three, Three, Two], bid: 765 },
        Hand { cards: vec![King, Jack, Jack, Ten, Ten], bid: 220 },
        Hand { cards: vec![King, King, Seven, Seven, Six], bid: 28 },
        Hand { cards: vec![Jack, Ten, Five, Five, Five], bid: 684 },
        Hand { cards: vec![Ace, Queen, Queen, Queen, Jack], bid: 483 },
    ]);
}

#[test]
fn test_part1() {
    assert_eq!(part_1(SAMPLE_INPUT.to_string()), 6440)
}
