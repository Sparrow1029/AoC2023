use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    thread,
};

use rust_aoc2023::{
    get_puzzle_input_string,
    grid::{Direction, Grid2D, Point},
};

type Visited = HashSet<(usize, Direction)>;

#[derive(Debug)]
struct GridError;

impl std::fmt::Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error in the grid")
    }
}

impl std::error::Error for GridError {}

// newtype pattern! wrap the foreign type in a new one to get around Rust's orphan rules
// https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
#[derive(Debug)]
struct MirrorGrid(Grid2D<Tile>);

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
            // Logic branches to determine new laser directions & positions
            match tile {
                Tile::Mirror(mirror) => match mirror {
                    '\\' => match self.dir {
                        Direction::Up | Direction::Down => {
                            self.dir = self.dir.turn_left();
                            self.pos = self.pos + self.dir.into();
                        }
                        Direction::Left | Direction::Right => {
                            self.dir = self.dir.turn_right();
                            self.pos = self.pos + self.dir.into();
                        }
                    },
                    '/' => match self.dir {
                        Direction::Up | Direction::Down => {
                            self.dir = self.dir.turn_right();
                            self.pos = self.pos + self.dir.into();
                        }
                        Direction::Left | Direction::Right => {
                            self.dir = self.dir.turn_left();
                            self.pos = self.pos + self.dir.into();
                        }
                    },
                    _ => panic!("bad mirror character: {mirror}"),
                },
                Tile::Splitter(split) => match split {
                    '-' => match self.dir {
                        // Keep going same direction
                        Direction::Left | Direction::Right => {
                            self.pos = self.pos + self.dir.into();
                        }
                        // Split off a new thread for a new laser beam
                        Direction::Up | Direction::Down => {
                            // Continue one beam going left
                            self.dir = Direction::Left;
                            self.pos = self.pos + self.dir.into();

                            // split off new beam going right
                            let mut new_laser = Laser {
                                dir: Direction::Right,
                                pos: self.pos + Direction::Right.into(),
                            };
                            let new_mutex = Arc::clone(&visited);
                            thread::scope(|s| {
                                let t = s.spawn(move || new_laser.beam(grid, new_mutex));
                                _ = t.join();
                            });
                        }
                    },
                    '|' => match self.dir {
                        // Keep going same direction
                        Direction::Up | Direction::Down => {
                            self.pos = self.pos + self.dir.into();
                        }
                        // Split off another thread for a new laser beam
                        Direction::Left | Direction::Right => {
                            // continue this beam going down
                            self.dir = Direction::Down;
                            self.pos = self.pos + self.dir.into();

                            // split off new beam going up
                            let mut new_laser = Laser {
                                dir: Direction::Up,
                                pos: self.pos + Direction::Up.into(),
                            };
                            let new_mutex = Arc::clone(&visited);
                            thread::scope(|s| {
                                let t = s.spawn(move || new_laser.beam(grid, new_mutex));
                                _ = t.join();
                            });
                        }
                    },
                    _ => panic!("bad splitter character: {split}"),
                },
                Tile::Empty => {
                    self.pos = self.pos + self.dir.into();
                }
            }
        }
    }
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

fn run_laser_simulation(grid: &MirrorGrid, start_pos: Point, start_direction: Direction) -> usize {
    let mut laser = Laser {
        dir: start_direction,
        pos: start_pos,
    };
    let visited: HashSet<(usize, Direction)> = HashSet::new();
    let visited_mutex = Arc::new(Mutex::new(visited));

    let first_clone = Arc::clone(&visited_mutex);
    let final_clone = Arc::clone(&visited_mutex);

    thread::scope(|s| {
        s.spawn(|| {
            laser.beam(grid, first_clone);
        });
    });
    let final_visited = final_clone.lock().unwrap();
    let deduplicated: HashSet<&usize> =
        HashSet::from_iter(final_visited.iter().map(|(pos, _)| pos));
    deduplicated.len()
}

fn part_1(grid: &MirrorGrid) -> usize {
    run_laser_simulation(grid, Point::new(0, 0), Direction::Right)
}

fn part_2(grid: &MirrorGrid) -> usize {
    let mut energized = vec![];
    for x in 0..grid.width {
        energized.push(thread::scope(|_| {
            run_laser_simulation(grid, Point::new(x, 0), Direction::Down)
        }));
        energized.push(thread::scope(|_| {
            run_laser_simulation(grid, Point::new(x, grid.height - 1), Direction::Up)
        }));
    }
    for y in 0..grid.height {
        energized.push(thread::scope(|_| {
            run_laser_simulation(grid, Point::new(0, y), Direction::Right)
        }));
        energized.push(thread::scope(|_| {
            run_laser_simulation(grid, Point::new(grid.width - 1, y), Direction::Left)
        }));
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
