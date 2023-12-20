use std::{collections::HashMap, fmt::Display};

use rust_aoc2023::{
    get_puzzle_input_string,
    grid::Grid2D,
    point::{Direction, Point},
};

type Cache = HashMap<Vec<Entity>, Vec<Entity>>;

#[derive(Debug)]
struct Map {
    grid: Grid2D<char>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl Map {
    fn is_valid_move(&self, point: Point) -> bool {
        self.grid.in_bounds(point)
            && match self.grid.get_ref(point) {
                Some(c) => *c != '#',
                None => false,
            }
    }

    fn render_with_entities(&self, entities: &[Entity]) {
        let mut grid_clone = self.grid.clone();
        entities.iter().for_each(|e| {
            grid_clone.set(e.pos, e.symbol);
        });
        println!("{grid_clone}");
    }
    fn get_load(&self, entities: &[Entity]) -> i64 {
        entities.iter().map(|e| self.grid.height - e.pos.y).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entity {
    pos: Point,
    symbol: char,
    direction: Direction,
}

impl Entity {
    fn new(pos: impl Into<Point>, direction: Direction, symbol: char) -> Self {
        Self {
            pos: pos.into(),
            symbol,
            direction,
        }
    }

    fn get_next_pos(&self) -> Point {
        self.pos + self.direction.into()
    }

    fn move_while_valid(&mut self, map: &Map, other_positions: &[Point]) {
        let mut next_pos = self.get_next_pos();
        while map.is_valid_move(next_pos) && !other_positions.contains(&next_pos) {
            self.pos = next_pos;
            next_pos = self.get_next_pos();
        }
    }
}

fn move_entities(map: &Map, entities: &mut [Entity]) {
    let mut cur_positions: Vec<Point> = entities.iter().map(|c| c.pos).collect();
    for (i, e) in entities.iter_mut().enumerate() {
        e.move_while_valid(map, &cur_positions);
        cur_positions[i] = e.pos;
    }
}

fn sort_by_direction(entities: &mut [Entity]) {
    let dir = entities.first().unwrap().direction;
    // println!("cur dir: {dir:?}");
    match dir {
        Direction::Up => entities.sort_by(|a, b| a.pos.y.cmp(&b.pos.y)),
        Direction::Down => entities.sort_by(|a, b| b.pos.y.cmp(&a.pos.y)),
        Direction::Left => entities.sort_by(|a, b| a.pos.x.cmp(&b.pos.x)),
        Direction::Right => entities.sort_by(|a, b| b.pos.x.cmp(&a.pos.x)),
    }
}

fn move_entities_cycles(map: &Map, entities: &mut [Entity], cycles: usize, cache: &mut Cache) {
    let mut cur_entities = entities.to_vec();
    sort_by_direction(&mut cur_entities);
    for i in 0..cycles {
        // Check cache for this particular configuration of rocks
        if let Some(res) = cache.get(&cur_entities) {
            println!("found same configuration at cycle {i}");
            map.render_with_entities(&cur_entities);
            cur_entities = res.clone();
            continue;
        } else {
            let orig_entities = cur_entities.clone();
            for _ in 0..4 {
                // map.render_with_entities(&cur_entities);
                sort_by_direction(&mut cur_entities);
                let mut positions: Vec<Point> = cur_entities.iter().map(|e| e.pos).collect();
                // for pos in positions.iter() {
                //     print!("{pos}, ");
                // }
                // println!();
                cur_entities.iter_mut().enumerate().for_each(|(i, e)| {
                    e.move_while_valid(map, &positions);
                    e.direction = e.direction.turn_left();
                    positions[i] = e.pos;
                });
            }
            cache.insert(orig_entities, cur_entities.clone());
        }
    }
}

fn parse_map(input: &str) -> (Map, Vec<Entity>) {
    let mut grid: Grid2D<char> = input.into();
    let mut entities = vec![];
    // Find all the round rocks (`O`) and replace with empty tiles ('.')
    // we return them as entities
    grid.find_all('O').unwrap().iter().for_each(|pos| {
        entities.push(Entity::new(*pos, Direction::Up, 'O'));
        grid.set(*pos, '.');
    });
    (Map { grid }, entities)
}

fn part_1(input: &str) {
    let (map, mut entities) = parse_map(input);
    move_entities(&map, &mut entities);
    println!("Part 1: {}", map.get_load(&entities));

    let mut cache: Cache = HashMap::new();
    let (map2, mut entities2) = parse_map(input);
    move_entities_cycles(&map2, &mut entities2, 1000, &mut cache);
}

fn main() {
    let input = get_puzzle_input_string(14).expect("I/O Error");
    part_1(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_parse_input() {
        let (map, entities) = parse_map(SAMPLE);
        println!("{map}");
        map.render_with_entities(&entities);
    }

    #[test]
    fn test_tilt_up() {
        let (map, mut entities) = parse_map(SAMPLE);
        move_entities(&map, &mut entities);
        map.render_with_entities(&entities);
        assert_eq!(map.get_load(&entities), 136);
    }

    #[test]
    fn test_cycle() {
        let (map, mut entities) = parse_map(SAMPLE);
        let mut cache: Cache = HashMap::new();
        move_entities_cycles(&map, &mut entities, 100, &mut cache);
        // map.render_with_entities(&new_entities);
        // println!("cache: {cache:?}");
    }
}
