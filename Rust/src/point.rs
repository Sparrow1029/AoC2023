use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub},
};

const NEIGHBORS_4: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
];

const NEIGHBORS_8: [Point; 8] = [
    Point { x: -1, y: 0 },
    Point { x: -1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        let (x, y) = value;
        Point::new(x, y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn from_index(i: usize, width: i64) -> Self {
        Point {
            x: i as i64 % width,
            y: i as i64 / width,
        }
    }

    pub fn manhattan_distance(&self, other: &Point) -> i64 {
        self.x.sub(other.x).abs() + self.y.sub(other.y).abs()
    }

    /// Euclidean distance between cartesian coordinates, rounded to two decimal places
    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        let res = ((self.x.sub(other.x).abs() as f64).powi(2)
            + (self.y.sub(other.y).abs() as f64).powi(2))
        .sqrt();
        (res * 100.).round() / 100.
    }

    pub fn get_4_neighbors(&self) -> [Point; 4] {
        NEIGHBORS_4.map(|n| n + *self)
    }

    pub fn get_8_neighbors(&self) -> [Point; 8] {
        NEIGHBORS_8.map(|n| n + *self)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
