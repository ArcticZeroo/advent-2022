use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Range;
use std::cmp::max;
use itertools::Itertools;
use regex::Regex;
use crate::common::movement::{Bounds, Grid, GridDirection, Point};
use crate::common::read_input;

const CHAMBER_WIDTH: usize = 7;
const CHAMBER_RANGE_X: Range<usize> = 0..CHAMBER_WIDTH;
const LEFT_OFFSET_X: usize = 2;
const BOTTOM_OFFSET_Y: usize = 3;
const DEFAULT_ROCK_OFFSET: Point = Point { x: LEFT_OFFSET_X as i128, y: BOTTOM_OFFSET_Y as i128 };
const PATTERNS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

struct RockChamber {
    rocks: HashSet<Point>,
    floor_y: u128,
    jet_pattern: Vec<GridDirection>,
    jet_pattern_index: usize,
}

impl RockChamber {
    pub fn new(jet_pattern: Vec<GridDirection>) -> RockChamber {
        RockChamber {
            rocks: HashSet::new(),
            floor_y: 0,
            jet_pattern_index: 0,
            jet_pattern,
        }
    }
}

fn parse_input(input: &str) -> Vec<GridDirection> {
    input.chars().map(|c| match c {
        '>' => GridDirection::Right,
        '<' => GridDirection::Left,
        _ => panic!()
    }).collect()
}

fn parse_patterns() -> VecDeque<Vec<Point>> {
    PATTERNS.split("\n\n")
        .map(|pattern| {
            pattern.split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .rev()
                .enumerate()
                .flat_map(|(y, line)| line
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '#' => Some(Point { x: x as i128, y: y as i128 }),
                        _ => None
                    })
                    .collect::<Vec<Point>>()
                ).collect::<Vec<Point>>()
        }).collect::<VecDeque<Vec<Point>>>()
}

fn is_legal_position(chamber: &RockChamber, rock_positions: &Vec<Point>) -> bool {
    rock_positions.iter().all(|point| {
        !chamber.rocks.contains(point) && CHAMBER_RANGE_X.contains(&(point.x as usize)) && point.y >= 0
    })
}

fn render_possible_grid(chamber: &RockChamber, positions: &Vec<Point>) {
    let max_y = max(chamber.floor_y, positions.iter().map(|point| point.y as u128).max().unwrap());
    let position_set: HashSet<&Point> = HashSet::from_iter(positions.iter());
    for y in (0..=max_y).rev() {
        println!("|{}|", (0..CHAMBER_WIDTH).map(|x| {
            let point = Point { x: x as i128, y: y as i128 };
            if position_set.contains(&point) {
                '@'
            } else if chamber.rocks.contains(&point) {
                '#'
            } else {
                '.'
            }
        }).join(""));
    }
    println!("+-------+");
    println!();
}

fn place_rock(chamber: &mut RockChamber, pattern: &Vec<Point>) {
    let current_rock_offset = Point { x: DEFAULT_ROCK_OFFSET.x, y: DEFAULT_ROCK_OFFSET.y + chamber.floor_y as i128 };
    let mut current_rock_positions: Vec<Point> = pattern.iter().map(|point| point.add(&current_rock_offset)).collect();

    loop {
        let next_jet_push = chamber.jet_pattern[chamber.jet_pattern_index];
        let jet_pushed_positions: Vec<Point> = current_rock_positions.iter().map(|point| point.get_moved_in_dir(next_jet_push)).collect();
        chamber.jet_pattern_index = (chamber.jet_pattern_index + 1) % chamber.jet_pattern.len();

        if is_legal_position(chamber, &jet_pushed_positions) {
            current_rock_positions = jet_pushed_positions;
        }

        let moved_down_positions: Vec<Point> = current_rock_positions.iter().map(|point| point.get_moved_in_dir(GridDirection::Down)).collect();
        // determine if we have landed on something/are at rest
        if !is_legal_position(chamber, &moved_down_positions) {
            for point in current_rock_positions {
                chamber.floor_y = max(chamber.floor_y, point.y as u128 + 1);
                chamber.rocks.insert(point);
            }
            return;
        }

        current_rock_positions = moved_down_positions;
    }
}

fn render_grid(chamber: &RockChamber) {
    println!();
    for y in (0..=chamber.floor_y).rev() {
        println!("|{}|", (0..CHAMBER_WIDTH).map(|x| if chamber.rocks.contains(&Point { x: x as i128, y: y as i128 }) { '#' } else { '.' }).join(""));
    }
    println!("+-------+");
}

fn part1(input: &str) -> u128 {
    let mut patterns = parse_patterns();
    let jet_patterns = parse_input(input);
    let mut chamber = RockChamber::new(jet_patterns);
    for _ in 0..2022 {
        let pattern = patterns.pop_front().unwrap();
        place_rock(&mut chamber, &pattern);
        patterns.push_back(pattern);
    }
    chamber.floor_y
}

#[derive(Debug)]
struct LoopData {
    start_iteration_index: u128,
    height_y: u128,
    iteration_count: u128,
    height_offset: u128,
}

struct ChamberConfiguration {
    iteration: u128,
    floor_y: u128
}

const ROW_CHECK_HEIGHT: u128 = 50;

fn serialize_chamber(chamber: &mut RockChamber, pattern_index: usize) -> String {
    format!("jet@{};patt@{};rows@{}", chamber.jet_pattern_index, pattern_index,
            (max(0, chamber.floor_y - ROW_CHECK_HEIGHT)..=chamber.floor_y)
                .map(|y| (0..CHAMBER_WIDTH).map(|x| if chamber.rocks.contains(&Point { x: x as i128, y: y as i128 }) { '#' } else { '.' }).join(""))
                .join("")
    )
}

fn find_loop(patterns: &VecDeque<Vec<Point>>, chamber: &mut RockChamber) -> LoopData {
    let mut configurations: HashMap<String, ChamberConfiguration> = HashMap::new();
    let mut loop_index: u128 = 0;
    let mut pattern_index = 0;
    loop {
        if loop_index >= ROW_CHECK_HEIGHT {
            let configuration = serialize_chamber(chamber, pattern_index);
            if let Some(existing_configuration) = configurations.get(&configuration) {
                return LoopData {
                    start_iteration_index: existing_configuration.iteration,
                    height_y: chamber.floor_y - existing_configuration.floor_y,
                    iteration_count: loop_index - existing_configuration.iteration,
                    height_offset: existing_configuration.floor_y
                }
            }

            configurations.insert(configuration, ChamberConfiguration {
                iteration: loop_index,
                floor_y: chamber.floor_y
            });
        }

        place_rock(chamber, &patterns[pattern_index]);
        pattern_index = (pattern_index + 1) % patterns.len();

        loop_index += 1;
    }
}

const TARGET_LOOP: u128 = 1_000_000_000_000;

fn part2(input: &str) -> u128 {
    let mut patterns = parse_patterns();
    let jet_patterns = parse_input(input);
    let mut chamber = RockChamber::new(jet_patterns);
    let loop_data = find_loop(&patterns, &mut chamber);
    let y_before_extra = chamber.floor_y;
    for _ in 0..((TARGET_LOOP - loop_data.start_iteration_index) % loop_data.iteration_count) {
        let pattern = patterns.pop_front().unwrap();
        place_rock(&mut chamber, &pattern);
        patterns.push_back(pattern);
    }
    let missed_loops = (TARGET_LOOP - loop_data.start_iteration_index) / loop_data.iteration_count;
    (chamber.floor_y - y_before_extra) + loop_data.height_offset + (loop_data.height_y * missed_loops)
}

pub fn run() {
    println!("Day 17");
    let input = read_input(17);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    pub fn part1() {
        assert_eq!(3068, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(1514285714288, super::part2(INPUT));
    }
}
