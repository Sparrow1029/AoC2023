use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    thread,
};

use rust_aoc2023::{
    get_puzzle_input_string,
    grid::Grid2D,
    point::{Direction, Point},
};

type Visited = HashSet<(usize, Direction)>;

// newtype pattern! wrap the foreign type in a new one to get around Rust's orphan rules
// https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
#[derive(Debug)]
struct MirrorGrid(Grid2D<Tile>);

/// Tiles for the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Mirror(char),
    Splitter(char),
    Empty,
}

/// Entity representing a laser beam at grid coord `pos` heading in `Direction` `dir`
#[derive(Debug)]
struct Laser {
    dir: Direction,
    pos: Point,
}

// Make it easier to access inner Grid2D<Tile> methods/fields
impl Deref for MirrorGrid {
    type Target = Grid2D<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Make it easier to access inner Grid2D<Tile> methods/fields
impl DerefMut for MirrorGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for MirrorGrid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len() as i64;
        let height = value.lines().count() as i64;
        let cells = value
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '/' | '\\' => Tile::Mirror(c),
                    '-' | '|' => Tile::Splitter(c),
                    '.' => Tile::Empty,
                    _ => panic!("Unrecognized char: {c}"),
                })
            })
            .collect();
        MirrorGrid(Grid2D {
            cells,
            width,
            height,
        })
    }
}

impl Laser {
    /// Move the laser beam tile by tile, turning when encountering reflecting mirrors
    /// and splitting off a separate scoped thread when a split is encountered.
    fn beam(&mut self, grid: &MirrorGrid, visited: Arc<Mutex<Visited>>) {
        // `None` is returned when the current point is out of bounds
        while let Some(tile) = grid.get_ref(self.pos) {
            {
                // Stop calculating when (position index, Direction) is encountered again to avoid loops
                let mut visited = visited.lock().expect("error getting mutex lock on HashSet");
                if !visited.insert((grid.pt_to_idx(self.pos), self.dir)) {
                    break;
                }
            }
            // determine new laser directions & positions
            match tile {
                Tile::Mirror(mirror) => self.reflect_beam(mirror),
                Tile::Splitter(splitter) => self.split_beam(splitter, grid, visited.clone()),
                Tile::Empty => {}
            }
            self.pos += self.dir.into();
        }
    }

    /// Handle mirrors which reflect the beam 90 degrees
    fn reflect_beam(&mut self, mirror: &char) {
        match mirror {
            '\\' => match self.dir {
                Direction::Up | Direction::Down => {
                    self.dir = self.dir.turn_left();
                }
                Direction::Left | Direction::Right => {
                    self.dir = self.dir.turn_right();
                }
            },
            '/' => match self.dir {
                Direction::Up | Direction::Down => {
                    self.dir = self.dir.turn_right();
                }
                Direction::Left | Direction::Right => {
                    self.dir = self.dir.turn_left();
                }
            },
            _ => unreachable!(),
        }
    }

    /// Handle splitters, which if oriented the same direction as the traveling laser, have no
    /// effect, but if perpendicular, split the beam in two.
    /// This is achieved by spinning up a new scoped thread for a beam traveling perpendicular,
    /// to the current one, and continuing the current beam in the opposite direction.
    fn split_beam(&mut self, splitter: &char, grid: &MirrorGrid, visited: Arc<Mutex<Visited>>) {
        match splitter {
            '-' => match self.dir {
                // same direction
                Direction::Left | Direction::Right => {}
                Direction::Up | Direction::Down => {
                    // Continue one beam going left
                    self.dir = Direction::Left;

                    // split off new beam going right
                    let mut new_laser = Laser {
                        dir: Direction::Right,
                        pos: self.pos + Direction::Right.into(),
                    };
                    thread::scope(|s| {
                        s.spawn(move || new_laser.beam(grid, visited.clone()));
                    });
                }
            },
            '|' => match self.dir {
                Direction::Up | Direction::Down => {}
                // Split off another thread for a new laser beam
                Direction::Left | Direction::Right => {
                    // continue this beam going down
                    self.dir = Direction::Down;

                    // split off new beam going up
                    let mut new_laser = Laser {
                        dir: Direction::Up,
                        pos: self.pos + Direction::Up.into(),
                    };
                    thread::scope(|s| {
                        s.spawn(move || new_laser.beam(grid, visited.clone()));
                    });
                }
            },
            _ => unreachable!(),
        }
    }
}

/// Determine how many tiles are "energized" (have a laser beam traveling through them)
fn run_laser_simulation(grid: &MirrorGrid, start_pos: Point, start_direction: Direction) -> usize {
    let mut laser = Laser {
        dir: start_direction,
        pos: start_pos,
    };
    let visited: Arc<Mutex<HashSet<(usize, Direction)>>> = Arc::new(Mutex::new(HashSet::new()));

    thread::scope(|s| {
        s.spawn(|| {
            laser.beam(grid, visited.clone());
        });
    });
    let final_visited = visited.lock().unwrap();
    let deduplicated: HashSet<&usize> =
        HashSet::from_iter(final_visited.iter().map(|(pos, _)| pos));
    deduplicated.len()
}

/// Determine how many tiles are energized with one beam starting from top left heading right.
fn part_1(grid: &MirrorGrid) -> usize {
    run_laser_simulation(grid, Point::new(0, 0), Direction::Right)
}

/// Determine the maximum number of tiles energized for any beam heading into the grid
/// from any x or y position.
fn part_2(grid: &MirrorGrid) -> usize {
    let mut energized = vec![];
    for x in 0..grid.width {
        energized.push(run_laser_simulation(
            grid,
            Point::new(x, 0),
            Direction::Down,
        ));
        energized.push(run_laser_simulation(
            grid,
            Point::new(x, grid.height - 1),
            Direction::Up,
        ));
    }
    for y in 0..grid.height {
        energized.push(run_laser_simulation(
            grid,
            Point::new(0, y),
            Direction::Right,
        ));
        energized.push(run_laser_simulation(
            grid,
            Point::new(grid.width - 1, y),
            Direction::Left,
        ));
    }
    energized.into_iter().max().unwrap()
}

fn main() {
    let grid: MirrorGrid = get_puzzle_input_string(16)
        .expect("I/O Error")
        .as_str()
        .into();
    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_parse() {
        let grid: MirrorGrid = SAMPLE.into();
        // println!("{grid:?}");
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn test_part_1() {
        let grid: MirrorGrid = SAMPLE.into();
        assert_eq!(part_1(&grid), 46);
    }

    #[test]
    fn test_part_2() {
        let grid: MirrorGrid = SAMPLE.into();
        assert_eq!(part_2(&grid), 51);
    }
}
