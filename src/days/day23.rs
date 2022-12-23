use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::common::movement::{Bounds, GridDirection, Point};
use crate::common::read_input;

fn parse_input(input: &str) -> HashSet<Point> {
    input.lines().rev().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some(Point { x: x as i128, y: y as i128 })
            } else {
                None
            }
        })
    }).collect()
}

fn get_neighbors(point: &Point) -> Vec<Point> {
    let Point { x, y } = *point;
    vec![
        Point { x: x - 1, y },
        Point { x: x - 1, y: y - 1 },
        Point { x: x - 1, y: y + 1 },
        Point { x: x + 1, y },
        Point { x: x + 1, y: y - 1 },
        Point { x: x + 1, y: y + 1 },
        Point { x, y: y + 1 },
        Point { x, y: y - 1 },
    ]
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DirectionWithDiagonal {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl DirectionWithDiagonal {
    pub fn all() -> Vec<DirectionWithDiagonal> {
        vec![
            DirectionWithDiagonal::N,
            DirectionWithDiagonal::NE,
            DirectionWithDiagonal::E,
            DirectionWithDiagonal::SE,
            DirectionWithDiagonal::S,
            DirectionWithDiagonal::SW,
            DirectionWithDiagonal::W,
            DirectionWithDiagonal::NW,
        ]
    }

    pub fn south_directions() -> Vec<DirectionWithDiagonal> {
        vec![
            DirectionWithDiagonal::S,
            DirectionWithDiagonal::SE,
            DirectionWithDiagonal::SW,
        ]
    }

    pub fn north_directions() -> Vec<DirectionWithDiagonal> {
        vec![
            DirectionWithDiagonal::N,
            DirectionWithDiagonal::NE,
            DirectionWithDiagonal::NW,
        ]
    }

    pub fn east_directions() -> Vec<DirectionWithDiagonal> {
        vec![
            DirectionWithDiagonal::E,
            DirectionWithDiagonal::NE,
            DirectionWithDiagonal::SE,
        ]
    }

    pub fn west_directions() -> Vec<DirectionWithDiagonal> {
        vec![
            DirectionWithDiagonal::W,
            DirectionWithDiagonal::NW,
            DirectionWithDiagonal::SW,
        ]
    }

    pub fn get_directions_from_grid_direction(dir: GridDirection) -> Vec<DirectionWithDiagonal> {
        match dir {
            GridDirection::Up => DirectionWithDiagonal::north_directions(),
            GridDirection::Down => DirectionWithDiagonal::south_directions(),
            GridDirection::Left => DirectionWithDiagonal::west_directions(),
            GridDirection::Right => DirectionWithDiagonal::east_directions(),
        }
    }

    pub fn move_point(&self, point: &Point) -> Point {
        match self {
            DirectionWithDiagonal::N => point.get_moved_in_dir(GridDirection::Up),
            DirectionWithDiagonal::NE => point.get_moved_in_dir(GridDirection::Up).get_moved_in_dir(GridDirection::Right),
            DirectionWithDiagonal::E => point.get_moved_in_dir(GridDirection::Right),
            DirectionWithDiagonal::SE => point.get_moved_in_dir(GridDirection::Down).get_moved_in_dir(GridDirection::Right),
            DirectionWithDiagonal::S => point.get_moved_in_dir(GridDirection::Down),
            DirectionWithDiagonal::SW => point.get_moved_in_dir(GridDirection::Down).get_moved_in_dir(GridDirection::Left),
            DirectionWithDiagonal::W => point.get_moved_in_dir(GridDirection::Left),
            DirectionWithDiagonal::NW => point.get_moved_in_dir(GridDirection::Up).get_moved_in_dir(GridDirection::Left),
        }
    }
}

fn does_dir_have_elves(map: &HashSet<Point>, elf_pos: &Point, directions: Vec<DirectionWithDiagonal>) -> bool {
    directions.iter().any(|dir| {
        let point_in_dir = dir.move_point(elf_pos);
        map.contains(&point_in_dir)
    })
}

const GRID_DIRECTIONS_IN_ORDER: [GridDirection; 4] = [
    GridDirection::Up, // north
    GridDirection::Down, // south
    GridDirection::Left, // west
    GridDirection::Right, // east
];

fn get_elf_proposition(map: &HashSet<Point>, round: u128, elf_pos: &Point) -> Option<Point> {
    let elves_by_direction: HashSet<DirectionWithDiagonal> = DirectionWithDiagonal::all()
        .iter()
        .filter_map(|dir| {
            let point_in_dir = dir.move_point(elf_pos);
            if map.contains(&point_in_dir) {
                Some(*dir)
            } else {
                None
            }
        }).collect();

    if elves_by_direction.is_empty() {
        return None;
    }

    for i in 0..GRID_DIRECTIONS_IN_ORDER.len() {
        let direction = GRID_DIRECTIONS_IN_ORDER[(i + round as usize) % GRID_DIRECTIONS_IN_ORDER.len()];
        if !does_dir_have_elves(map, elf_pos, DirectionWithDiagonal::get_directions_from_grid_direction(direction)) {
            return Some(elf_pos.get_moved_in_dir(direction));
        }
    }

    None
}

fn did_elf_move_during_round(map: &mut HashSet<Point>, round: u128) -> bool {
    let mut elves_by_proposed_movement: HashMap<Point, Vec<Point>> = HashMap::new();

    for elf_pos in map.iter() {
        if let Some(proposed_point) = get_elf_proposition(map, round, elf_pos) {
            elves_by_proposed_movement.entry(proposed_point).or_insert_with(|| vec![]).push(*elf_pos);
        }
    }

    if elves_by_proposed_movement.is_empty() {
        return false;
    }

    for (proposed_point, elves_proposing) in elves_by_proposed_movement {
        if elves_proposing.len() != 1 {
            continue;
        }

        let elf_proposing = elves_proposing.first().unwrap();
        map.remove(elf_proposing);
        map.insert(proposed_point);
    }

    true
}

struct Rect {
    x_bounds: Bounds<i128>,
    y_bounds: Bounds<i128>,
}

fn find_elf_rect(map: &HashSet<Point>) -> Rect {
    let mut x_bounds: Bounds<i128> = Bounds::new(0, 0);
    let mut y_bounds: Bounds<i128> = Bounds::new(0, 0);

    for elf_pos in map {
        x_bounds.update(elf_pos.x);
        y_bounds.update(elf_pos.y);
    }

    Rect {
        x_bounds,
        y_bounds,
    }
}

fn render_map(map: &HashSet<Point>) {
    let elf_rect = find_elf_rect(map);
    println!("{}",
             elf_rect.y_bounds.to_range().rev().map(|y|
                 elf_rect.x_bounds.to_range().map(|x| {
                     let point = Point { x, y };
                     if map.contains(&point) {
                         '#'
                     } else {
                         '.'
                     }
                 }).join("")
             ).join("\n")
    );
}

fn part1(input: &str) -> u128 {
    let mut map = parse_input(input);

    for i in 0..10 {
        did_elf_move_during_round(&mut map, i);
    }

    let elf_rect = find_elf_rect(&map);
    elf_rect.y_bounds.to_range().map(|y|
        elf_rect.x_bounds.to_range().filter(|&x| {
            let point = Point { x, y };
            !map.contains(&point)
        }).count() as u128
    ).sum()
}

fn part2(input: &str) -> u128 {
    let mut map = parse_input(input);

    let mut i = 0;
    loop {
        if !did_elf_move_during_round(&mut map, i) {
            return i + 1;
        }
        i += 1;
    }
}

pub fn run() {
    println!("Day 23");
    let input = read_input(23);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

//     const INPUT: &str = ".....
// ..##.
// ..#..
// .....
// ..##.
// .....";

    #[test]
    pub fn part1() {
        assert_eq!(110, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(20, super::part2(INPUT));
    }
}
