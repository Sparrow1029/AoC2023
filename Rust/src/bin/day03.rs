use rust_aoc2023::{get_puzzle_input_string, Result};
use std::collections::HashSet;

const DAY: u32 = 3;
const NEIGHBOR_COORDS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];
type Coord = (i32, i32);

#[derive(Debug)]
enum GridCell {
    Num(u16),
    Symbol(char),
    Empty,
}

#[derive(Debug)]
struct Grid {
    w: i32,
    h: i32,
    cells: Vec<GridCell>,
}

impl Grid {
    /// Check if provided (x, y) coordinate is in the bounds of the `Grid`
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (x < self.w && x >= 0) && (y < self.h && y >= 0)
    }

    /// Return a reference to a GridCell
    fn get(&self, x: i32, y: i32) -> Option<&GridCell> {
        if !self.in_bounds(x, y) {
            return None;
        }
        Some(&self.cells[y as usize * self.w as usize + x as usize])
    }

    /// Return set of all in-bounds coordinates surrounding a point
    fn get_neighbor_coords(&self, x: usize, y: usize) -> Vec<(i32, i32)> {
        NEIGHBOR_COORDS
            .iter()
            .map(|(nx, ny)| (x as i32 + nx, y as i32 + ny))
            .collect()
    }

    /// Return a number created by joining horizontal digits
    fn gen_number_from_coord(&self, coord: Coord) -> (usize, HashSet<Coord>) {
        let (x, y) = coord;
        let start_num = if let Some(GridCell::Num(n)) = self.get(x, y) {
            n
        } else {
            panic!("Should have a number")
        };
        let mut digits: Vec<u16> = vec![*start_num];
        let mut coord_set = HashSet::from_iter(vec![coord]);
        // search left
        for lx in (x - 3..x).rev() {
            match self.get(lx, y) {
                Some(GridCell::Num(n)) => {
                    digits.insert(0, *n);
                    coord_set.insert((lx, y));
                }
                Some(_) | None => break,
            }
        }
        // search right
        for rx in x + 1..x + 3 {
            match self.get(rx, y) {
                Some(GridCell::Num(n)) => {
                    digits.push(*n);
                    coord_set.insert((rx, y));
                }
                Some(_) | None => break,
            }
        }
        (
            digits.into_iter().fold(0, |acc, i| acc * 10 + i) as usize,
            coord_set,
        )
    }

    fn get_adjacent_numbers(&self, coord: Coord, visited: &mut HashSet<Coord>) -> Vec<usize> {
        let mut nums: Vec<usize> = vec![];
        let (x, y) = coord;
        let adjacent_num_coords: Vec<Coord> = self
            .get_neighbor_coords(x as usize, y as usize)
            .iter()
            .filter_map(|(nx, ny)| match self.get(*nx, *ny) {
                Some(GridCell::Num(_)) => Some((*nx, *ny)),
                _ => None,
            })
            .collect();

        for coord in adjacent_num_coords {
            if visited.contains(&coord) {
                continue;
            }
            let (val, visited_coords) = self.gen_number_from_coord(coord);
            visited.extend(visited_coords);
            nums.push(val);
        }
        nums
    }
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
                '0'..='9' => cells.push(GridCell::Num(val.to_digit(10).unwrap() as u16)),
                '.' => cells.push(GridCell::Empty),
                _ => cells.push(GridCell::Symbol(val)),
            }
        }
    }
    Ok(Grid {
        w: w as i32,
        h: h as i32,
        cells,
    })
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
                let x = i as i32 % grid.w;
                let y = i as i32 / grid.w;
                grid.get_adjacent_numbers((x, y), &mut visited)
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
                    Some((i as i32 % grid.w, i as i32 / grid.w))
                } else {
                    None
                }
            }
            _ => None,
        })
        .filter_map(|coord| {
            let nums = grid.get_adjacent_numbers(coord, &mut visited);
            match nums.len() {
                2 => Some(nums.iter().product::<usize>()),
                _ => None,
            }
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
