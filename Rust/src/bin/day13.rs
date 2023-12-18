use std::cmp::Ordering;

use rust_aoc2023::get_puzzle_input_string;

fn parse_grid_to_bin(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut rows = vec![];
    let mut cols = vec![0; 32];
    for line in input.lines() {
        let mut cur_row = 0;
        for (i, c) in line.chars().enumerate() {
            cur_row <<= 1;
            cols[i] <<= 1;
            match c {
                '#' => {
                    cur_row |= 1;
                    cols[i] |= 1;
                }
                '.' => {}
                _ => panic!("unrecognized character"),
            }
        }
        // println!("{cur_row:09b}");
        rows.push(cur_row);
    }
    (rows, cols.into_iter().filter(|v| *v > 0).collect())
}

fn fix_smudge(array: &[usize]) -> Vec<Vec<usize>> {
    let mut new_arrays = vec![];
    'outer: for i in 0..array.len() - 1 {
        let mut new_vec = array.to_vec();
        for j in i + 1..new_vec.len() {
            let (left, right) = (new_vec[i], new_vec[j]);
            let xor = left ^ right;
            if xor.count_ones() == 1 {
                println!("cmp {left:010b} ({left}) -> {right:010b} ({right}) xor.count_ones is 1");
                match left.cmp(&right) {
                    Ordering::Greater => new_vec[i] ^= xor,
                    Ordering::Less => new_vec[j] ^= xor,
                    Ordering::Equal => unreachable!(),
                }
                println!("Fixed new_vec {new_vec:?}");
                println!("Original:");
                for v in array.iter() {
                    println!("\t{v:010b}");
                }
                println!("New:");
                for n in &new_vec {
                    println!("\t{n:010b}");
                }
                new_arrays.push(new_vec);
                continue 'outer;
            }
        }
    }
    new_arrays
}

fn get_mirror_val(array: &[usize]) -> Option<usize> {
    'outer: for i in 0..array.len() - 1 {
        if array[i] == array[i + 1] {
            let (left, right) = array.split_at(i + 1);
            println!("array {left:?} <> {right:?}");
            for (j, val) in left.iter().rev().enumerate().skip(1) {
                println!("{j}:{val}");
                if j < right.len() && *val != right[j] {
                    continue 'outer;
                }
            }
            return Some(i + 1);
        }
    }
    None
}

fn detect_mirror(rows: &[usize], cols: &[usize]) -> usize {
    match get_mirror_val(rows) {
        Some(idx) => 100 * idx,
        None => match get_mirror_val(cols) {
            Some(idx) => idx,
            None => panic!("no mirror detected"),
        },
    }
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grid| {
            let (rows, cols) = parse_grid_to_bin(grid);
            println!("\n{grid}\nrows {rows:?}\ncols {cols:?}");
            detect_mirror(&rows, &cols)
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let res: Vec<(Vec<usize>, Vec<usize>)> = input
        .split("\n\n")
        .map(|grid| {
            let (rows, cols) = parse_grid_to_bin(grid);
            // println!("\n{grid}\nrows {rows:?}\ncols {cols:?}");
            (rows, cols)
        })
        .collect();
    println!("res {res:?}");
    for (rows, cols) in res.into_iter() {
        println!("ROWS");
        fix_smudge(&rows);
        println!("COLS");
        fix_smudge(&cols);
        println!("\n\n");
    }
    0
}

fn main() {
    let input = get_puzzle_input_string(13).expect("I/O Error");
    println!("Part 1: {}", part_1(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const SAMPLE_2: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_parse() {
        let (rows, cols) = parse_grid_to_bin(SAMPLE_1);
        assert_eq!(rows, vec![358, 90, 385, 385, 90, 102, 346]);
        assert_eq!(cols, vec![89, 24, 103, 66, 37, 37, 66, 103, 24]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_2), 405);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_2), 400);
    }
}
