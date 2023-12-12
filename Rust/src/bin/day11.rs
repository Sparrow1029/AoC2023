use rust_aoc2023::{
    get_puzzle_input_string,
    grid::{Grid2D, Point},
};

fn expand_galactic_map(grid: &Grid2D<char>, expansion_factor: i64) -> Vec<Point> {
    // get (x, y) coordinates of all galaxies
    let mut galaxy_pts = grid.find_all('#').unwrap();
    let factor = expansion_factor - 1; // adjust for off-by-one
                                       // get empty cols x
    let cols: Vec<i64> = (0..grid.width)
        .filter(|x| grid.get_column(*x).unwrap().iter().all(|c| *c == '.'))
        .collect();
    // get empty rows y
    let rows: Vec<i64> = (0..grid.height)
        .filter(|y| grid.get_row(*y).unwrap().iter().all(|c| *c == '.'))
        .collect();

    // adjust x & y coordinates to account for expansion of rows & columns
    for (i, col) in cols.iter().enumerate() {
        let col_with_offset = (i as i64 * factor) + col;
        galaxy_pts.iter_mut().for_each(|pt| {
            if pt.x > col_with_offset {
                pt.x += factor;
            }
        });
    }
    for (i, row) in rows.iter().enumerate() {
        let row_with_offset = (i as i64 * factor) + row;
        galaxy_pts.iter_mut().for_each(|pt| {
            if pt.y > row_with_offset {
                pt.y += factor;
            }
        });
    }
    galaxy_pts
}

fn sum_shortest_distances(points: &[Point]) -> i64 {
    let mut total = 0;
    for (i, pt1) in points.iter().enumerate() {
        for pt2 in &points[i + 1..] {
            total += pt1.manhattan_distance(pt2);
        }
    }
    total
}

fn main() {
    let input_string = get_puzzle_input_string(11).expect("I/O Error");
    let grid: Grid2D<char> = input_string.as_str().into();
    println!(
        "Part 1: {}",
        sum_shortest_distances(&expand_galactic_map(&grid, 2))
    );
    println!(
        "Part 2: {}",
        sum_shortest_distances(&expand_galactic_map(&grid, 1_000_000))
    );
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
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
    fn test_part1() {
        let grid: Grid2D<char> = Grid2D::from(INPUT);
        let galaxy_pts = expand_galactic_map(&grid, 2);
        println!("Modified points: {galaxy_pts:?}");
        let sum = sum_shortest_distances(&galaxy_pts);
        assert_eq!(sum, 374);
    }

    #[test]
    fn test_part2() {
        let grid: Grid2D<char> = Grid2D::from(INPUT);
        let galaxy_pts = expand_galactic_map(&grid, 10);
        println!("Modified points: {galaxy_pts:?}");
        assert_eq!(sum_shortest_distances(&galaxy_pts), 1030);
        let galaxy_pts_2 = expand_galactic_map(&grid, 100);
        assert_eq!(sum_shortest_distances(&galaxy_pts_2), 8410);
    }
}
