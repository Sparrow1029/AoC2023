use crate::Result;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
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

    pub fn get_4_neighbors(&self) -> [Point; 4] {
        NEIGHBORS_4.map(|n| n + *self)
    }

    pub fn get_8_neighbors(&self) -> [Point; 8] {
        NEIGHBORS_8.map(|n| n + *self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid2D<T> {
    pub width: i64,
    pub height: i64,
    pub cells: Vec<T>,
}

impl<T> Grid2D<T>
where
    T: PartialEq + Clone + Copy,
{
    /// Take index of usize and return a Point value (grid in bounds)
    fn idx_to_point(&self, idx: usize) -> Option<Point> {
        if idx > self.width as usize * self.height as usize {
            return None;
        }
        let y = idx as i64 / self.width;
        let x = idx as i64 % self.width;
        Some(Point::new(x, y))
    }

    /// Find index in self.cells vec for _in_bounds_(!) point
    fn pt_to_idx(&self, p: Point) -> usize {
        (p.y * self.width + p.x) as usize
    }

    /// Check if provided (x, y) coordinate is in the bounds of the `Grid`
    pub fn in_bounds(&self, p: Point) -> bool {
        (p.x < self.width && p.x >= 0) && (p.y < self.height && p.y >= 0)
    }

    /// Return a reference to a GridCell
    pub fn get_ref(&self, p: Point) -> Option<&T> {
        if !self.in_bounds(p) {
            return None;
        }
        Some(&self.cells[self.pt_to_idx(p)])
    }

    /// Return a mutable reference to a GridCell
    pub fn get_mut_ref(&mut self, p: Point) -> Option<&mut T> {
        if !self.in_bounds(p) {
            return None;
        }
        Some(&mut self.cells[(p.y * self.width + p.x) as usize])
    }

    /// Return set of all in-bounds coordinates surrounding a point
    pub fn get_neighbor8_coords(&self, p: Point) -> Vec<Point> {
        p.get_8_neighbors()
            .into_iter()
            .filter(|pt| self.in_bounds(*pt))
            .collect()
    }

    /// Get ref to all 8 cells surrounding a point
    pub fn get_neighbor8_ref_cells(&self, p: Point) -> Vec<&T> {
        self.get_neighbor8_coords(p)
            .iter()
            .filter_map(|coord| self.get_ref(*coord))
            .collect()
    }

    /// Return set of all N, S, E, W in-bounds coordinates surrounding a point
    pub fn get_neighbor4_coords(&self, p: Point) -> Vec<Point> {
        p.get_4_neighbors()
            .into_iter()
            .filter(|pt| self.in_bounds(*pt))
            .collect()
    }

    /// Find (x, y) coord of item if present
    pub fn find_item_coord(&self, item: &T) -> Option<Point> {
        if let Some(idx) = self.cells.iter().position(|cell| cell == item) {
            return self.idx_to_point(idx);
        }
        None
    }

    /// Return vec of `Point`s where element is found. None if element is not found.
    pub fn find_all(&self, element: T) -> Option<Vec<Point>> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if item == &element {
                    return Some(self.idx_to_point(idx));
                }
                None
            })
            .collect()
    }

    /// Return a slice representing a row in the grid
    pub fn get_row(&self, y: i64) -> Option<&[T]> {
        if !self.in_bounds(Point::new(0, y)) {
            return None;
        }
        let start_idx = (y * self.width) as usize;
        Some(&self.cells[start_idx..start_idx + self.width as usize])
    }

    /// Return a slice representing a column in the grid
    pub fn get_column(&self, x: i64) -> Option<Vec<T>> {
        if !self.in_bounds(Point::new(x, 0)) {
            return None;
        }
        let column = (x as usize..self.cells.len())
            .step_by(self.width as usize)
            .map(|idx| self.cells[idx])
            .collect::<Vec<T>>();
        Some(column)
    }

    /// Add column to grid.cells
    pub fn insert_column(&mut self, x: i64, element: T) -> Result<()> {
        self.width += 1;
        for y in 0..self.height {
            let new_idx = self.pt_to_idx((x, y).into());
            self.cells.insert(new_idx, element)
        }
        Ok(())
    }

    /// Add row to grid.cells
    pub fn insert_row(&mut self, y: i64, element: T) -> Result<()> {
        let start_idx = y * self.width;
        for x in start_idx..start_idx + self.width {
            self.cells.insert(x as usize, element)
        }
        self.height += 1;
        Ok(())
    }
}

impl From<&str> for Grid2D<char> {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let iter = value.lines().collect::<Vec<&str>>();
        let height = iter.len();
        let cells: Vec<char> = iter.into_iter().flat_map(|c| c.chars()).collect();
        Grid2D {
            cells,
            width: width as i64,
            height: height as i64,
        }
    }
}

impl<T> std::fmt::Display for Grid2D<T>
where
    T: PartialEq + Clone + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get_ref(Point::new(x, y)).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_insert_row() {
        let mut grid: Grid2D<char> = TEST_INPUT1.into();
        _ = grid.insert_row(3, '.');
        println!("{grid}");
        assert!(grid.get_row(3).unwrap().iter().all(|c| *c == '.'));
    }
}
