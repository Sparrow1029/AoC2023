use std::{
    fmt::Display,
    ops::{Add, Sub},
};

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
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
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

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn from_index(i: usize, width: i32) -> Self {
        Point {
            x: i as i32 % width,
            y: i as i32 / width,
        }
    }
}

#[derive(Debug)]
pub struct Grid2D<T> {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<T>,
}

impl<T> Grid2D<T> {
    /// Check if provided (x, y) coordinate is in the bounds of the `Grid`
    pub fn in_bounds(&self, p: Point) -> bool {
        (p.x < self.width && p.x >= 0) && (p.y < self.height && p.y >= 0)
    }

    /// Return a reference to a GridCell
    pub fn get_ref(&self, p: Point) -> Option<&T> {
        if !self.in_bounds(p) {
            return None;
        }
        Some(&self.cells[(p.y * self.width + p.x) as usize])
    }
    pub fn get_mut_ref(&mut self, p: Point) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None;
        }
        Some(&mut self.cells[(p.y * self.width + p.x) as usize])
    }

    /// Return set of all in-bounds coordinates surrounding a point
    pub fn get_neighbor8_coords(&self, p: Point) -> Vec<Point> {
        NEIGHBORS_8
            .iter()
            .filter_map(|np| {
                let coord = p + *np;
                if self.in_bounds(coord) {
                    Some(coord)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get ref to all cells surrounding a point
    pub fn get_neighbor8_ref_cells(&self, p: Point) -> Vec<&T> {
        self.get_neighbor8_coords(p)
            .iter()
            .filter_map(|coord| self.get_ref(*coord))
            .collect()
    }
}
