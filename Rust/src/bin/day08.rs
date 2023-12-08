use rust_aoc2023::{get_puzzle_input_string, lcm};
use std::collections::HashMap;

const DAY_08: u32 = 8;

#[derive(Debug, PartialEq, Eq)]
enum Next {
    Left,
    Right,
}

impl From<char> for Next {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unrecognized character"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Next>, HashMap<&str, (&str, &str)>) {
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let split = input.split("\n\n").collect::<Vec<&str>>();

    for line in split[1].lines() {
        // Example line:
        // AAA = (BBB, CCC)
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        node_map.insert(key, (left, right));
    }
    (split[0].chars().map(Next::from).collect(), node_map)
}

fn traverse_map(
    start_node: &str,
    node_map: &HashMap<&str, (&str, &str)>,
    instructions: &[Next],
) -> u64 {
    let mut steps = 0usize;
    let mut node = start_node;
    let i_len = instructions.len();
    while !(node.ends_with('Z')) {
        let (left, right) = node_map[node];
        match instructions[steps % i_len] {
            Next::Left => node = left,
            Next::Right => node = right,
        }
        steps += 1;
    }
    steps as u64
}

fn part_1(instructions: &[Next], node_map: &HashMap<&str, (&str, &str)>) -> u64 {
    traverse_map("AAA", node_map, instructions)
}

fn part_2(instructions: &[Next], node_map: &HashMap<&str, (&str, &str)>) -> u64 {
    let start_nodes = node_map
        .keys()
        .filter(|s| s.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();
    let all_path_lengths = start_nodes
        .into_iter()
        .map(|p| traverse_map(p, node_map, instructions))
        .collect::<Vec<u64>>();
    lcm(&all_path_lengths)
}

fn main() {
    let input = get_puzzle_input_string(DAY_08).expect("I/O Error");
    let (instructions, node_map) = parse_input(&input);

    println!("Part 1: {}", part_1(&instructions, &node_map));
    println!("Part 2: {}", part_2(&instructions, &node_map));
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part_1, Next};

    const SAMPLE_INPUT: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse_input() {
        let (instructions, node_map) = parse_input(SAMPLE_INPUT);
        assert_eq!(instructions, vec![Next::Left, Next::Left, Next::Right]);
        assert_eq!(node_map["BBB"], ("AAA", "ZZZ"));
    }

    #[test]
    fn test_part_1() {
        let (instructions, node_map) = parse_input(SAMPLE_INPUT);
        assert_eq!(part_1(&instructions, &node_map), 6);
    }
}
