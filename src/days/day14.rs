use std::ops::RangeInclusive;
use itertools::{all, Itertools};
use num_traits::abs;
use crate::common;
use crate::common::movement::{Grid, GridDirection, Point};
use crate::common::read_input;

enum GridElement {
    Sand,
    RockWall,
}

type SandGrid = Grid<GridElement>;

const SAND_FILL_SOURCE: Point = Point { x: 500, y: 0 };

fn has_visited(grid: &SandGrid, point: &Point, floor_y_opt: Option<i128>) -> bool {
    grid.has_visited(point) || match floor_y_opt {
        None => false,
        Some(floor_y) => point.y <= floor_y
    }
}

fn get_sand_resting_point(grid: &SandGrid, floor_y_opt: Option<i128>) -> Option<Point> {
    let grid_x_bounds = grid.get_x_bounds().to_range();
    let grid_y_bounds = grid.get_y_bounds();
    let mut current_point = SAND_FILL_SOURCE.clone();
    loop {
        // Sand cannot fall forever with floor enabled
        if floor_y_opt.is_none() && (!grid_x_bounds.contains(&current_point.x) || current_point.y < grid_y_bounds.min) {
            return None;
        }

        let below = current_point.get_moved_in_dir(GridDirection::Down);

        if !has_visited(&grid, &below, floor_y_opt) {
            current_point = below;
            continue;
        }

        let below_left = below.get_moved_in_dir(GridDirection::Left);
        if !has_visited(&grid, &below_left, floor_y_opt) {
            current_point = below_left;
            continue;
        }

        let below_right = below.get_moved_in_dir(GridDirection::Right);
        if !has_visited(&grid, &below_right, floor_y_opt) {
            current_point = below_right;
            continue;
        }

        break;
    }
    Some(current_point)
}

fn range_between(a: i128, b: i128) -> RangeInclusive<i128> {
    if a > b {
        b..=a
    } else {
        a..=b
    }
}

fn straight_line_between(a: &Point, b: &Point) -> Vec<Point> {
    if a.y == b.y {
        range_between(a.x, b.x).map(|x| Point { x, y: a.y }).collect()
    } else {
        range_between(a.y, b.y).map(|y| Point { x: a.x, y }).collect()
    }
}

fn parse_input(input: &str) -> SandGrid {
    let mut grid = Grid::new();

    for line in input.split("\n") {
        let points: Vec<Point> = line.split(" -> ").map(|point_raw| {
            let (x, y) = point_raw.split_once(',').unwrap();
            Point { x: x.parse::<i128>().unwrap(), y: -(y.parse::<i128>().unwrap()) }
        }).collect();

        assert!(points.len() > 1);

        let mut last_point = points.first().unwrap();
        for next_point in &points[1..] {
            for line_point in straight_line_between(last_point, next_point) {
                grid.visit(line_point, GridElement::RockWall);
            }
            last_point = next_point;
        }
    }

    grid
}

fn render_grid(grid: &SandGrid) -> String {
    grid.get_y_bounds().to_range().rev().map(|y| grid.get_x_bounds().to_range().map(|x| {
        let point = Point { x, y };
        match grid.get_value(&point) {
            Some(element) => match(element) {
                GridElement::RockWall => "#",
                GridElement::Sand => "o"
            },
            None => "."
        }
    }).join("")).join("\n")
}

fn part1(input: &str) -> u128 {
    let mut grid = parse_input(input);
    let mut sand_units_placed = 0;
    loop {
        if let Some(point) = get_sand_resting_point(&grid, None /*floor_y_opt*/) {
            grid.visit(point, GridElement::Sand);
            sand_units_placed += 1;
        } else {
            return sand_units_placed;
        }
    }
}

fn part2(input: &str) -> u128 {
    let mut grid = parse_input(input);
    let mut sand_units_placed = 0;
    let floor_y = grid.get_y_bounds().min - 2;
    loop {
        if let Some(point) = get_sand_resting_point(&grid, Some(floor_y)) {
            grid.visit(point, GridElement::Sand);
            sand_units_placed += 1;
            if point == SAND_FILL_SOURCE {
                return sand_units_placed;
            }
        } else {
            panic!("Sand should never be falling forever with floor enabled");
        }
    }
}

pub fn run() {
    println!("Day 14");
    let input = read_input(14);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    pub fn part1() {
        assert_eq!(24, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(93, super::part2(INPUT));
    }
}