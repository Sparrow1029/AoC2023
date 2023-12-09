use rust_aoc2023::get_puzzle_input_string;

fn parse_seq(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|d| d.parse::<i32>().unwrap())
        .collect()
}

fn differences(seq: &[i32]) -> Vec<i32> {
    (1..seq.len()).map(|idx| seq[idx] - seq[idx - 1]).collect()
}

fn get_back_value(seq: &[i32]) -> i32 {
    let last = seq.last().unwrap_or(&0);
    if seq.iter().all(|d| d == last) {
        *last
    } else {
        last + get_back_value(&differences(seq))
    }
}

fn get_front_value(seq: &[i32]) -> i32 {
    let first = seq.first().unwrap_or(&0);
    if seq.iter().all(|d| d == first) {
        *first
    } else {
        first - get_front_value(&differences(seq))
    }
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| get_back_value(&parse_seq(line)))
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| get_front_value(&parse_seq(line)))
        .sum()
}

fn main() {
    let input = get_puzzle_input_string(9).expect("I/O Error");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45 
";
    #[test]
    fn test_parse() {
        let seq = "0 3 6 9 12 15";
        assert_eq!(parse_seq(seq), vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_differences() {
        let seq = parse_seq("0 3 6 9 12 15");
        assert_eq!(differences(&seq), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_next_back() {
        let seq = parse_seq("0 3 6 9 12 15");
        assert_eq!(get_back_value(&seq), 18);
    }
    #[test]

    fn test_part_1() {
        assert_eq!(part_1(SAMPLE), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE), 2);
    }
}
