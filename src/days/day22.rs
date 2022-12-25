use std::collections::{HashMap, HashSet};
use std::iter::Map;
use itertools::Itertools;
use regex::Regex;
use crate::common::movement::{Bounds, GridDirection, Point};
use crate::common::{read_input, read_input_no_trim, wrap_value, wrap_value_around_bounds};

struct WrapGrid {
    // Overall bounds across the whole grid - no guarantee that these are in bounds
    overall_x_bounds: Bounds<i128>,
    overall_y_bounds: Bounds<i128>,
    // Actual bounds where points exist
    y_bounds_by_x: HashMap<i128, Bounds<i128>>,
    x_bounds_by_y: HashMap<i128, Bounds<i128>>,
    walls: HashSet<Point>,
}

const GRID_DIRECTIONS_IN_CLOCKWISE_ORDER: [GridDirection; 4] = [
    GridDirection::Up,
    GridDirection::Right,
    GridDirection::Down,
    GridDirection::Left
];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn turn_grid_direction(&self, grid_dir: GridDirection) -> GridDirection {
        let current_direction_index = GRID_DIRECTIONS_IN_CLOCKWISE_ORDER.iter().position(|value| grid_dir == *value).unwrap() as i128;
        let new_direction_index = match self {
            TurnDirection::Left => wrap_value(current_direction_index - 1, GRID_DIRECTIONS_IN_CLOCKWISE_ORDER.len() as i128),
            TurnDirection::Right => wrap_value(current_direction_index + 1, GRID_DIRECTIONS_IN_CLOCKWISE_ORDER.len() as i128),
        };
        GRID_DIRECTIONS_IN_CLOCKWISE_ORDER[new_direction_index as usize]
    }

    pub fn from_str(value: &str) -> TurnDirection {
        match value {
            "L" => TurnDirection::Left,
            "R" => TurnDirection::Right,
            _ => panic!()
        }
    }
}

struct MoveInstruction {
    turn_dir: Option<TurnDirection>,
    move_amount: u128,
}

struct ProgramInput {
    map: WrapGrid,
    instructions: Vec<MoveInstruction>,
}

fn add_to_bounds(bounds_by_coordinate: &mut HashMap<i128, Bounds<i128>>, coordinate: i128, bound_coordinate: i128) {
    bounds_by_coordinate.entry(coordinate).or_insert_with(|| Bounds::new(0, 0)).update(bound_coordinate)
}

fn parse_map(input: &str) -> WrapGrid {
    let mut grid = WrapGrid {
        overall_x_bounds: Bounds::new(0, 0),
        overall_y_bounds: Bounds::new(0, 0),
        x_bounds_by_y: HashMap::new(),
        y_bounds_by_x: HashMap::new(),
        walls: HashSet::new(),
    };

    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point { x: x as i128, y: y as i128 };
            match c {
                '#' | '.' => {
                    grid.overall_x_bounds.update(point.x);
                    grid.overall_y_bounds.update(point.y);
                    add_to_bounds(&mut grid.x_bounds_by_y, point.y, point.x);
                    add_to_bounds(&mut grid.y_bounds_by_x, point.x, point.y);
                    if c == '#' {
                        grid.walls.insert(point);
                    }
                }
                _ => ()
            }
        }
    }

    grid
}

fn parse_input(input: &str) -> ProgramInput {
    let turn_instruction_regex = Regex::new(r"(?P<count>\d+)(?P<turn>[LR])?").unwrap();

    let (map_str, instructions_str) = input.split_once("\n\n").unwrap();

    let map = parse_map(map_str);

    let instructions: Vec<MoveInstruction> = turn_instruction_regex.captures_iter(instructions_str).map(|capture| {
        MoveInstruction {
            turn_dir: capture.name("turn").map(|turn_match| TurnDirection::from_str(turn_match.as_str())),
            move_amount: capture.name("count").unwrap().as_str().parse().unwrap(),
        }
    }).collect();

    ProgramInput {
        map,
        instructions,
    }
}

struct MapState {
    position: Point,
    direction: GridDirection,
}

fn execute_instruction(last_dir_by_point: &mut HashMap<Point, GridDirection>, map: &WrapGrid, state: MapState, instruction: &MoveInstruction) -> MapState {
    let mut current_state = MapState {
        position: state.position,
        direction: state.direction,
    };

    last_dir_by_point.insert(current_state.position, current_state.direction);

    for i in 0..instruction.move_amount {
        let raw_moved_point = current_state.position.get_moved_in_dir(state.direction);

        let x_bounds = map.x_bounds_by_y.get(&current_state.position.y).unwrap();
        let y_bounds = map.y_bounds_by_x.get(&current_state.position.x).unwrap();

        let bounded_moved_point = Point {
            x: wrap_value_around_bounds(raw_moved_point.x, x_bounds),
            y: wrap_value_around_bounds(raw_moved_point.y, y_bounds),
        };

        // If we're up against the wall, no point in continuing movement, no more turns may occur.
        if map.walls.contains(&bounded_moved_point) {
            break;
        }

        current_state.position = bounded_moved_point;

        last_dir_by_point.insert(current_state.position, current_state.direction);
    }

    if let Some(turn_dir) = instruction.turn_dir {
        current_state.direction = turn_dir.turn_grid_direction(current_state.direction);
    }

    current_state
}

fn direction_value(direction: GridDirection) -> i128 {
    match direction {
        GridDirection::Right => 0,
        GridDirection::Down => 1,
        GridDirection::Left => 2,
        GridDirection::Up => 3
    }
}

fn render_grid(map: &WrapGrid, last_dir_by_point: &HashMap<Point, GridDirection>) {
    let rendered_grid = map.overall_y_bounds.to_range().rev().map(|y| {
        map.overall_x_bounds.to_range().map(|x| {
            if !map.y_bounds_by_x.get(&x).unwrap().to_range().contains(&y) || !map.x_bounds_by_y.get(&y).unwrap().to_range().contains(&x) {
                ' '
            } else {
                let point = Point { x, y };

                let last_dir_for_this_point_opt = last_dir_by_point.get(&point);
                if let Some(last_dir_for_this_point) = last_dir_for_this_point_opt {
                    return match last_dir_for_this_point {
                        GridDirection::Up => '^',
                        GridDirection::Down => 'v',
                        GridDirection::Left => '<',
                        GridDirection::Right => '>'
                    };
                }

                if map.walls.contains(&point) {
                    '#'
                } else {
                    '.'
                }
            }
        }).join("")
    }).join("\n");

    println!("{}", rendered_grid);
}

fn part1(input: &str) -> i128 {
    let ProgramInput { mut map, instructions } = parse_input(input);
    let max_y = map.overall_y_bounds.max;
    let mut current_state = MapState {
        direction: GridDirection::Right,
        // leftmost tile of the top row of tiles
        position: Point {
            y: max_y,
            x: map.x_bounds_by_y.get(&max_y).unwrap().min,
        },
    };
    let mut last_dir_by_point = HashMap::new();
    for instruction in &instructions {
        current_state = execute_instruction(&mut last_dir_by_point, &map, current_state, instruction);
    }
    // render_grid(&map, &last_dir_by_point);
    (1000 * ((map.overall_y_bounds.max - current_state.position.y) + 1)) + (4 * (current_state.position.x + 1)) + direction_value(current_state.direction)
}

fn part2(input: &str) -> u128 {
    0
}

pub fn run() {
    println!("Day 22");
    let input = read_input_no_trim(22);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    pub fn part1() {
        assert_eq!(6032, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(20, super::part2(INPUT));
    }
}
