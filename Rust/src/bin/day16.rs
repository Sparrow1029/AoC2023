use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
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
    visited: HashSet<usize>,
}

impl Laser {
    // fn beam(&mut self, grid: &MirrorGrid, visited: Arc<Mutex<Visited>>) {
    fn beam(&mut self, tx: Sender<HashSet<usize>>, grid: &MirrorGrid) {
        'outer: while let Some(tile) = grid.get_ref(self.pos) {
            println!("At tile {tile:?} - {}", self.pos);
            thread::sleep(std::time::Duration::from_millis(100));
            self.visited.insert(grid.pt_to_idx(self.pos));
            match tile {
                Tile::Mirror(mirror) => match mirror {
                    '\\' => match self.dir {
                        Direction::Up | Direction::Down => {
                            self.dir = self.dir.turn_left();
                            self.pos = self.pos + self.dir.into();
                            continue 'outer;
                        }
                        Direction::Left | Direction::Right => {
                            self.dir = self.dir.turn_right();
                            self.pos = self.pos + self.dir.into();
                            continue 'outer;
                        }
                    },
                    '/' => match self.dir {
                        Direction::Up | Direction::Down => {
                            self.dir = self.dir.turn_right();
                            self.pos = self.pos + self.dir.into();
                            continue 'outer;
                        }
                        Direction::Left | Direction::Right => {
                            self.dir = self.dir.turn_left();
                            self.pos = self.pos + self.dir.into();
                            continue 'outer;
                        }
                    },
                    _ => panic!("bad mirror character: {mirror}"),
                },
                Tile::Splitter(split) => match split {
                    '-' => match self.dir {
                        // Keep going same direction
                        Direction::Left | Direction::Right => {
                            self.pos = self.pos + self.dir.into();
                            continue 'outer;
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
                                visited: HashSet::new(),
                            };
                            let tx_clone = tx.clone();
                            thread::scope(|s| {
                                let t = s.spawn(move || new_laser.beam(tx_clone, grid));
                                _ = t.join();
                            });
                            continue 'outer;
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
                                visited: HashSet::new(),
                            };
                            let tx_clone = tx.clone();
                            thread::scope(|s| {
                                let t = s.spawn(move || new_laser.beam(tx_clone, grid));
                                _ = t.join();
                            });
                            continue 'outer;
                        }
                    },
                    _ => panic!("bad splitter character: {split}"),
                },
                Tile::Empty => {
                    self.pos = self.pos + self.dir.into();
                }
            }
        }
        println!("Out of bounds!");
        _ = tx.send(self.visited.clone());
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

fn part_1(input: &str) -> usize {
    let grid: MirrorGrid = input.into();
    let (tx, rx) = channel::<HashSet<usize>>();
    let tx_cloned = tx.clone();
    // let grid_arc = Arc::clone(&grid);
    let mut laser = Laser {
        dir: Direction::Right,
        pos: (0, 0).into(),
        visited: HashSet::new(),
    };
    thread::scope(|s| {
        let t = s.spawn(|| {
            laser.beam(tx_cloned, &grid);
        });
        _ = t.join();
    });
    let mut visited: HashSet<(usize, Direction)> = HashSet::new();
    let visited_mutex = Arc::new(Mutex::new(visited));
    let mut all_visited: HashSet<usize> = HashSet::new();

    while let Ok(set) = rx.recv() {
        println!("we made it!");
        all_visited.extend(set.iter());
        // all_visited = all_visited.union(&set).copied().collect();
    }
    println!("Final set: {all_visited:?}");
    all_visited.len()
}

fn main() {
    let input = get_puzzle_input_string(16).expect("I/O Error");
    println!("Part 1: {}", part_1(&input));
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
        part_1(SAMPLE);
    }
}
