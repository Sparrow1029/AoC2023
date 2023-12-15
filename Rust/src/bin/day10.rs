use colored::Colorize;
use rust_aoc2023::{
    get_puzzle_input_string,
    grid::{Grid2D, Point},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Start,
    Empty,
    NorthSouth,
    EastWest,
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::NorthSouth,
            '-' => Pipe::EastWest,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            'S' => Pipe::Start,
            '.' => Pipe::Empty,
            _ => panic!("invalid character: {value}"),
        }
    }
}

fn parse_pipe_grid(input: &str) -> Grid2D<Pipe> {
    let width = input.lines().peekable().peek().unwrap_or(&"").len() as i64;
    let cells: Vec<Pipe> = input
        .lines()
        .flat_map(|line| line.chars().map(Pipe::from))
        .collect();
    Grid2D {
        width,
        height: cells.len() as i64 / width,
        cells,
    }
}

fn get_next_pos(grid: &Grid2D<Pipe>, cur_pos: Point, dir: Direction) -> Option<(Point, Direction)> {
    match dir {
        Direction::North => {
            let next_pt = cur_pos + (0, -1).into();
            let next_pipe = grid.get_ref(next_pt).unwrap();
            let new_dir = match next_pipe {
                Pipe::Start => return None,
                Pipe::NorthSouth => Some(Direction::North),
                Pipe::SouthEast => Some(Direction::East),
                Pipe::SouthWest => Some(Direction::West),
                _ => panic!("pipe {next_pipe:?} can't connect"),
            };
            Some(new_dir).map(|v| (next_pt, v.unwrap()))
        }
        Direction::South => {
            let next_pt = cur_pos + (0, 1).into();
            let next_pipe = grid.get_ref(next_pt).unwrap();
            let new_dir = match next_pipe {
                Pipe::Start => return None,
                Pipe::NorthSouth => Some(Direction::South),
                Pipe::NorthEast => Some(Direction::East),
                Pipe::NorthWest => Some(Direction::West),
                _ => panic!("pipe {next_pipe:?} can't connect"),
            };
            Some(new_dir).map(|v| (next_pt, v.unwrap()))
        }
        Direction::East => {
            let next_pt = cur_pos + (1, 0).into();
            let next_pipe = grid.get_ref(next_pt).unwrap();
            let new_dir = match next_pipe {
                Pipe::Start => return None,
                Pipe::EastWest => Some(Direction::East),
                Pipe::NorthWest => Some(Direction::North),
                Pipe::SouthWest => Some(Direction::South),
                _ => panic!("pipe {next_pipe:?} can't connect"),
            };
            Some(new_dir).map(|v| (next_pt, v.unwrap()))
        }
        Direction::West => {
            let next_pt = cur_pos + (-1, 0).into();
            let next_pipe = grid.get_ref(next_pt).unwrap();
            let new_dir = match next_pipe {
                Pipe::Start => return None,
                Pipe::EastWest => Some(Direction::West),
                Pipe::NorthEast => Some(Direction::North),
                Pipe::SouthEast => Some(Direction::South),
                _ => panic!("pipe {next_pipe:?} can't connect"),
            };
            Some(new_dir).map(|v| (next_pt, v.unwrap()))
        }
    }
}

/// Tailored this to my puzzle input for brevity's sake
fn follow_loop(
    grid: &Grid2D<Pipe>,
    start_pt: Point,
    direction: Direction,
    loop_vertices: &mut Vec<Point>,
) -> usize {
    let mut count = 1;
    let (mut next_pt, mut next_dir) = (start_pt, direction);
    while let Some((pos, dir)) = get_next_pos(grid, next_pt, next_dir) {
        next_pt = pos;
        loop_vertices.push(next_pt);
        next_dir = dir;
        count += 1;
    }
    count
}

#[allow(dead_code)]
fn print_grid(grid: &Grid2D<Pipe>, color_pipe_positions: &[Point]) {
    let mut output = String::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pt = Point::new(x, y);
            let pipe = grid.get_ref(pt).unwrap();
            let pipe_symbol = match pipe {
                // Pipe::NorthSouth => "║",
                // Pipe::EastWest => "═",
                // Pipe::NorthEast => "╚",
                // Pipe::NorthWest => "╝",
                // Pipe::SouthWest => "╗",
                // Pipe::SouthEast => "╔",
                // Pipe::Start => "█",
                // Pipe::Empty => ".",
                Pipe::NorthSouth => "│",
                Pipe::EastWest => "─",
                Pipe::NorthEast => "└",
                Pipe::NorthWest => "┘",
                Pipe::SouthWest => "┐",
                Pipe::SouthEast => "┌",
                Pipe::Start => "█",
                Pipe::Empty => "☻",
            };
            if color_pipe_positions.contains(&pt) {
                output.push_str(format!("{}", pipe_symbol.red()).as_str());
            } else {
                output.push_str(pipe_symbol);
            };
        }
        output.push('\n');
    }
    println!("{output}");
}

fn shoelace_area(vertices: Vec<Point>) -> i64 {
    let mut vertices = vertices.clone();
    // add first point to end to ensure first point is considered against last point
    vertices.push(*vertices.first().unwrap());
    vertices
        .as_slice()
        .windows(2)
        .map(|pair| {
            let (pt1, pt2) = (pair[0], pair[1]);
            pt1.x * pt2.y - pt1.y * pt2.x
        })
        .sum::<i64>()
        .abs()
        / 2
}

fn part2(vertices: Vec<Point>) -> i64 {
    // For part two, stolen algorithms from Reddit:
    // Pick's theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
    // loop_area = interior_points_count + (boundary_points_count / 2) - 1
    //
    //  Part 2 answer is interior_points_count
    // transforming Pick's formula:
    // interior_points_count = loopArea - (boundary_points_count / 2) + 1
    //
    // boundary_points_count is length of loop
    //
    // loop_area can by calculated using Shoelace formula (https://en.wikipedia.org/wiki/Shoelace_formula):
    // vertices = (x1, y1) (x2, y2) (x3, y3) ...
    // 2 * loop_area = x1 * y2 - y1 * x2 + x2 * y3 - x3 * y2 + ...
    // loop_area = result / 2

    // print_grid(&grid, &loop_vertices);
    let vertices_len = vertices.len() as i64;
    let loop_area = shoelace_area(vertices);
    loop_area - vertices_len / 2 + 1
}

fn main() {
    let input = get_puzzle_input_string(10).expect("I/O Error");
    let grid = parse_pipe_grid(&input);
    let start_pt = grid.find_item_coord(&Pipe::Start).unwrap();
    // Laziness -- customized to my puzzle input
    let init_direction = Direction::West;
    // Vec to contain vertices as they are found
    let mut loop_vertices = vec![start_pt];

    println!(
        "Part 1: {}",
        follow_loop(&grid, start_pt, init_direction, &mut loop_vertices) / 2
    );
    println!("Part 2: {}", part2(loop_vertices));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
    const LARGE_SAMPLE: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const ANOTHER_SAMPLE: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    #[test]
    fn test_part1() {
        let grid = parse_pipe_grid(SAMPLE);
        let mut positions = vec![];
        let start_pt = grid.find_item_coord(&Pipe::Start).unwrap();
        let init_dir = Direction::South;
        assert_eq!(follow_loop(&grid, start_pt, init_dir, &mut positions), 16);
    }

    #[test]
    fn test_color() {
        let grid = parse_pipe_grid(LARGE_SAMPLE);
        let mut positions: Vec<Point> = vec![(12, 4).into()];
        let start_pt: Point = Point::new(12, 4);
        let _ = follow_loop(&grid, start_pt, Direction::East, &mut positions);
        print_grid(&grid, &positions);
    }

    #[test]
    fn test_part2() {
        let grid = parse_pipe_grid(LARGE_SAMPLE);
        let start_pt = grid.find_item_coord(&Pipe::Start).unwrap();
        let mut positions = vec![start_pt];
        let init_dir = Direction::East;
        _ = follow_loop(&grid, start_pt, init_dir, &mut positions);
        assert_eq!(part2(positions), 8);
    }

    #[test]
    fn test_part2_again() {
        let grid = parse_pipe_grid(ANOTHER_SAMPLE);
        let start_pt = grid.find_item_coord(&Pipe::Start).unwrap();
        let mut positions = vec![start_pt];
        let init_dir = Direction::West;
        _ = follow_loop(&grid, start_pt, init_dir, &mut positions);
        assert_eq!(part2(positions), 10);
    }
}
