use std::collections::BinaryHeap;

use rust_aoc2023::{grid::Grid2D, point::Point};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

fn parse_input(input: &str) -> Grid2D<u8> {
    let width = input.lines().next().unwrap().len() as i64;
    let height = input.lines().count() as i64;
    let cells = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect::<Vec<u8>>();
    Grid2D {
        width,
        height,
        cells,
    }
}

fn a_star_with_max_run(grid: &Grid2D<u8>) -> usize {
    // open_list   - priority queue of nodes to process (binary heap)
    // closed_list - set of processed nodes (hash set)
    //
    // track to make sure we only take up to 3 steps in one direction,
    // curr dir    - current direction of travel
    // cur_dir cnt - number of steps in current direction
    //
    // while open list is not empty:
    //     - pop lowest-cost node from open_list (binary heap fn)
    //     - get neighbors (NSWE) facing forward, left & right that
    //       have not already been processed (check closed_list)
    //
    //       if neighbor is the goal, stop search
    //
    42
}

fn main() {}

#[cfg(test)]
mod test {

    use super::*;

    const SAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_parse() {
        let grid = parse_input(SAMPLE);
        assert_eq!(grid.width, 13);
        assert_eq!(grid.get_ref((1, 1).into()).unwrap(), &2);
    }
}
