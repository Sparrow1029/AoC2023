use colored::Colorize;
use rust_aoc2023::{
    get_puzzle_input_string,
    grid::{Grid2D, Point},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
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
    loop_positions: &mut Vec<Point>,
) -> usize {
    let mut count = 1;
    let (mut next_pt, mut next_dir) = (start_pt, direction);
    while let Some((pos, dir)) = get_next_pos(grid, next_pt, next_dir) {
        next_pt = pos;
        loop_positions.push(next_pt);
        next_dir = dir;
        count += 1;
    }
    count
}

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

fn main() {
    let input = get_puzzle_input_string(10).expect("I/O Error");
    // print_grid(&input);
    let grid = parse_pipe_grid(&input);
    let start_pt = grid.find_item_coord(&Pipe::Start).unwrap();
    let init_direction = Direction::West;
    let mut loop_positions = vec![start_pt];

    println!(
        "Part 1: {}",
        follow_loop(&grid, start_pt, init_direction, &mut loop_positions) / 2
    );
    // For part two, I just print the grid and manually count hehe
    print_grid(&grid, &loop_positions);
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
    #[test]
    fn test_part1() {
        let grid = parse_pipe_grid(SAMPLE);
        let mut positions = vec![];
        print_grid(&grid, &vec![]);
        let start_pt = grid.find_item_coord(&Pipe::Start).unwrap();
        let init_dir = Direction::South;
        assert_eq!(follow_loop(&grid, start_pt, init_dir, &mut positions), 16);
    }

    #[test]
    fn test_fill() {
        let input = "\
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
        let grid = parse_pipe_grid(input);
        let mut positions: Vec<Point> = vec![(12, 4).into()];
        let start_pt: Point = Point::new(12, 4);
        let _ = follow_loop(&grid, start_pt, Direction::East, &mut positions);
        print_grid(&grid, &positions);
    }
}
