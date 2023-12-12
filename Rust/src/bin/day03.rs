use self::GridCell::*;
use rust_aoc2023::{
    get_puzzle_input_string,
    grid::{Grid2D, Point},
    Result,
};
use std::collections::HashSet;

const DAY: u32 = 3;
type Grid = Grid2D<GridCell>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridCell {
    Number(u16),
    Symbol(char),
    Empty,
}

/// Parse puzzle input into a "2-D Grid" (flat array) of `GridCell` Enum values
fn parse_input(input: String) -> Result<Grid> {
    let mut cells = Vec::with_capacity(1024);
    let puzzle_input = input.lines().collect::<Vec<_>>();
    let h = puzzle_input.len();
    let w = puzzle_input.iter().peekable().peek().unwrap().len();
    for line in puzzle_input {
        for val in line.chars() {
            match val {
                '0'..='9' => cells.push(GridCell::Number(val.to_digit(10).unwrap() as u16)),
                '.' => cells.push(GridCell::Empty),
                _ => cells.push(GridCell::Symbol(val)),
            }
        }
    }
    Ok(Grid2D {
        width: w as i64,
        height: h as i64,
        cells,
    })
}

/// Return a number created by joining adjacent horizontal digits in a grid
fn gen_number_from_pt(grid: &Grid, pt: Point) -> (usize, HashSet<Point>) {
    let start_num = if let Some(Number(n)) = grid.get_ref(pt) {
        n
    } else {
        panic!("Should be a number")
    };
    let mut digits: Vec<u16> = vec![*start_num];
    let mut coord_set = HashSet::from_iter(vec![pt]);
    // search left
    for lx in (pt.x - 3..pt.x).rev() {
        match grid.get_ref((lx, pt.y).into()) {
            Some(GridCell::Number(n)) => {
                digits.insert(0, *n);
                coord_set.insert((lx, pt.y).into());
            }
            Some(_) | None => break,
        }
    }
    // search right
    for rx in pt.x + 1..pt.x + 3 {
        match grid.get_ref((rx, pt.y).into()) {
            Some(GridCell::Number(n)) => {
                digits.push(*n);
                coord_set.insert((rx, pt.y).into());
            }
            Some(_) | None => break,
        }
    }
    (
        digits.into_iter().fold(0, |acc, i| acc * 10 + i) as usize,
        coord_set,
    )
}

/// Find coordinates of numbers adjacent to passed-in point
/// construct separate horizontal digits into whole numbers and return them
fn get_adjacent_numbers(grid: &Grid, pt: Point, visited: &mut HashSet<Point>) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    let adjacent_num_coords: Vec<Point> = grid
        .get_neighbor8_coords(pt)
        .iter()
        .filter_map(|new_pt| match grid.get_ref(*new_pt) {
            Some(GridCell::Number(_)) => Some(*new_pt),
            _ => None,
        })
        .collect();

    for coord in adjacent_num_coords {
        if visited.contains(&coord) {
            continue;
        }
        let (val, visited_coords) = gen_number_from_pt(grid, coord);
        visited.extend(visited_coords);
        nums.push(val);
    }
    nums
}

/// Iterate through the grid cells, getting the coordinates of
/// types `GridCell::Symbol`, find the numbers adjacent to them
/// then sum all of those numbers
fn solve_part_1(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    grid.cells
        .iter()
        .enumerate()
        .flat_map(|(i, c)| match c {
            GridCell::Symbol(_) => {
                get_adjacent_numbers(grid, Point::from_index(i, grid.width), &mut visited)
            }
            _ => vec![],
        })
        .sum::<usize>()
}

/// Find coordinates of all gears ('`*`' symbol)
/// filter for just ones that are touching exactly two numbers
/// multiply those two numbers
/// sum all the multiples
fn solve_part_2(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    grid.cells
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| match cell {
            GridCell::Symbol(s) => {
                if *s == '*' {
                    let coord = Point::from_index(i, grid.width);
                    let nums = get_adjacent_numbers(grid, coord, &mut visited);
                    match nums.len() {
                        2 => Some(nums.iter().product::<usize>()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum::<usize>()
}

fn main() -> Result<()> {
    let grid = parse_input(get_puzzle_input_string(DAY)?)?;
    println!("Part 1: {}", solve_part_1(&grid));
    println!("Part 2: {}", solve_part_2(&grid));
    Ok(())
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn test_part1() {
    let grid = parse_input(TEST_INPUT.to_string()).unwrap();
    // println!("{grid:?}");
    assert_eq!(solve_part_1(&grid), 4361);
}

#[test]
fn test_part2() {
    let grid = parse_input(TEST_INPUT.to_string()).unwrap();
    // println!("{grid:?}");
    assert_eq!(solve_part_2(&grid), 467835);
}
