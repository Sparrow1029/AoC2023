use rust_aoc2023::get_puzzle_input_string;
use std::ops::Range;

const DAY_05: u32 = 5;

type ResourceMap = Vec<Map>;

#[derive(Debug)]
struct SeedMap {
    seeds: Vec<u64>,
    maps: Vec<ResourceMap>,
}

#[derive(Debug)]
struct Map {
    seed_range: Range<u64>,
    value_range: Range<u64>,
    // offset: u64,
}

impl Map {
    fn new(value_start: u64, seed_start: u64, range: u64) -> Self {
        Map {
            seed_range: seed_start..seed_start + range + 1,
            value_range: value_start..value_start + range + 1,
            // offset: std::cmp::max(value_start, seed_start) - std::cmp::min(value_start, seed_start),
        }
    }

    /// Return a value if this mapped "range" contains the passed-in seed
    fn get_mapped_value(&self, seed: u64) -> Option<u64> {
        if self.seed_range.contains(&seed) {
            let offset = seed - self.seed_range.start;
            return Some(self.value_range.start + offset);
        }
        None
    }

    /// Return overlapping range if there is one, as well as the leftover
    /// ranges if there are any
    /// cases:\
    /// ```
    ///           |---|
    /// range     3 4 5
    /// seeds 1 2 3 4 5 6 7
    /// leftovers: []
    ///
    ///           |---|
    /// range     3 4 5 6 7
    /// seeds 1 2 3 4 5
    /// leftovers: [(6..8)]
    ///           |---|
    /// range 1 2 3 4 5
    /// seeds     3 4 5 6 7
    ///           |---|
    /// drnge 1 2 3 4 5
    /// destn     1 2 3 4 5
    /// leftovers: [(1..3)]
    /// ```
    fn get_overlaps(&self, ranges: &Vec<Range<u64>>) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        let mut overlaps = vec![];
        let mut leftovers = vec![];
        for range in ranges {
            let end_val = range.end;
            if self.seed_range.contains(&range.start) && self.seed_range.contains(&end_val) {
                let start = self.get_mapped_value(range.start).unwrap();
                let end = self.get_mapped_value(end_val).unwrap();
                overlaps.push(start..end);
            } else if self.seed_range.contains(&range.start) {
                let start = self.get_mapped_value(range.start).unwrap();
                leftovers.push(self.seed_range.end + 1..range.end);
                overlaps.push(start..self.value_range.end);
            } else if self.seed_range.contains(&end_val) {
                let end = self.get_mapped_value(end_val).unwrap();
                leftovers.push(range.start..self.seed_range.start + 1);
                overlaps.push(self.value_range.start..end);
            } else {
                leftovers.push(range.clone());
            }
        }
        (leftovers, overlaps)
    }
}

impl From<Vec<u64>> for Map {
    fn from(value: Vec<u64>) -> Self {
        assert!(value.len() == 3);
        Map::new(value[0], value[1], value[2])
    }
}

impl SeedMap {
    /// Find this map's value corresponding to a seed
    fn get_location_single_seed(&self, seed: u64) -> u64 {
        let mut cur_loc = seed;
        'outer: for mapvec in self.maps.iter() {
            for map in mapvec {
                if let Some(v) = map.get_mapped_value(cur_loc) {
                    cur_loc = v;
                    continue 'outer;
                }
            }
        }
        cur_loc
    }

    /// For a resource_map (Vec<Range<u64>>), get any overlaps and leftovers
    fn get_map_overlaps(&self, range: Range<u64>, resource_map: &ResourceMap) -> Vec<Range<u64>> {
        let mut solved = vec![];
        let mut unresolved = vec![range];
        for map in resource_map.iter() {
            let (leftovers, mut overlaps) = map.get_overlaps(&unresolved);
            unresolved = leftovers;
            solved.append(&mut overlaps);
        }
        solved.append(&mut unresolved);
        solved
    }

    fn get_min_location_for_range(&self, range: &Range<u64>) -> u64 {
        let mut current_stack = vec![range.clone()];
        for resource_map in &self.maps {
            let mut next_stack = vec![];
            while let Some(rng) = current_stack.pop() {
                next_stack.append(&mut self.get_map_overlaps(rng, resource_map));
            }
            current_stack = next_stack;
        }
        let min = current_stack.iter().map(|rng| rng.start).min().unwrap();
        min
    }

    fn get_min_location_for_all_ranges(&self) -> u64 {
        let mut min = u64::MAX;
        for i in (0..self.seeds.len()).step_by(2) {
            let pair = (self.seeds[i], self.seeds[i + 1]);
            let cmp_range = pair.0..pair.0 + pair.1 + 1;
            let check_min = self.get_min_location_for_range(&cmp_range);
            min = std::cmp::min(min, check_min);
        }
        min
    }
}

fn parse_input(input: String) -> SeedMap {
    // Get seed values
    let mut map_input = input.split("\n\n");
    let seeds = map_input
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse::<u64>().expect("NaN"))
        .collect::<Vec<u64>>();

    let mut maps: Vec<Vec<Map>> = vec![];
    for block in map_input {
        let mut cur_maps = vec![];
        for line in block.lines().skip(1) {
            let map = line
                .split_whitespace()
                .map(|d| d.parse::<u64>().expect("NaN"))
                .collect::<Vec<u64>>();
            cur_maps.push(map.into());
        }
        maps.push(cur_maps);
    }
    SeedMap { seeds, maps }
}

fn part1() {
    let input = get_puzzle_input_string(DAY_05).expect("error getting puzzle input");
    let seed_map = parse_input(input);
    let answer = seed_map
        .seeds
        .iter()
        .map(|s| seed_map.get_location_single_seed(*s))
        .min()
        .unwrap();
    println!("Part1: {answer}")
}

fn part2() {
    let input = get_puzzle_input_string(DAY_05).expect("error getting puzzle input");
    let seed_map = parse_input(input);
    let answer = seed_map.get_min_location_for_all_ranges();
    println!("Part 2: {answer}");
}

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const SAMPLE_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn test_parse_input() {
    let seed_map = parse_input(SAMPLE_INPUT.to_string());
    assert_eq!(seed_map.seeds.len(), 4);
    assert_eq!(seed_map.maps.len(), 7);
}

#[test]
fn test_get_seed_value() {
    let seed_map = parse_input(SAMPLE_INPUT.to_string());
    let test_map = &seed_map.maps[0][1];
    assert_eq!(test_map.get_mapped_value(79), Some(81));
}

#[test]
fn test_part1() {
    let seed_map = parse_input(SAMPLE_INPUT.to_string());
    let answer = seed_map
        .seeds
        .iter()
        .map(|s| seed_map.get_location_single_seed(*s))
        .min()
        .unwrap();
    assert_eq!(answer, 35);
}

#[test]
fn test_part2() {
    let seed_map = parse_input(SAMPLE_INPUT.to_string());
    let answer = seed_map.get_min_location_for_all_ranges();
    assert_eq!(answer, 46);
}
