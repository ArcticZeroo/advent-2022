use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::common::movement::{Bounds, Grid, GridDirection, Point};

use crate::common::{char_alphabet_position, read_input};

struct HillMap {
    start: Point,
    goal: Point,
    graph: Grid<u32>,
}

impl HillMap {
    fn get_accessible_neighbors(&self, point: &Point) -> Vec<Point> {
        let elevation = self.graph.get_value(point).unwrap();

        GridDirection::all().iter().filter_map(|&direction| {
            let mut point_in_dir = point.clone();
            point_in_dir.move_in_dir(direction);
            if let Some(neighbor_elevation) = self.graph.get_value(&point_in_dir) {
                if *neighbor_elevation <= (*elevation + 1) {
                    return Some(point_in_dir);
                }
            }
            return None;
        }).collect()
    }

    fn get_neighbors_that_could_access(&self, point: &Point) -> Vec<Point> {
        let elevation = self.graph.get_value(point).unwrap();

        GridDirection::all().iter().filter_map(|&direction| {
            let mut point_in_dir = point.clone();
            point_in_dir.move_in_dir(direction);
            if let Some(neighbor_elevation) = self.graph.get_value(&point_in_dir) {
                if *elevation <= (*neighbor_elevation + 1) {
                    return Some(point_in_dir);
                }
            }
            return None;
        }).collect()
    }

    pub fn get_all_points(&self) -> Vec<Point> {
        self.graph.get_y_bounds().to_range()
            .flat_map(|y| self.graph.get_x_bounds().to_range().map(|x| Point { x, y }).collect::<Vec<Point>>())
            .collect()
    }
}

fn parse_input(input: &str) -> HillMap {
    let lines: Vec<&str> = input.split("\n").collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut hill_map = HillMap {
        start: Point::zero(),
        goal: Point::zero(),
        graph: Grid::new(),
    };

    for y in 0..height {
        let line = lines[y];
        for (x, value) in line.chars().enumerate() {
            let position = Point { x: x as i128, y: y as i128 };

            let elevation = match value {
                'S' => {
                    hill_map.start = position.clone();
                    char_alphabet_position('a', true /*is_lowercase*/)
                }
                'E' => {
                    hill_map.goal = position.clone();
                    char_alphabet_position('z', true /*is_lowercase*/)
                }
                _ => char_alphabet_position(value, true /*is_lowercase*/)
            };

            hill_map.graph.visit(position, elevation);
        }
    }

    assert_eq!(Bounds::new(0, width as i128 - 1), hill_map.graph.get_x_bounds());
    assert_eq!(Bounds::new(0, height as i128 - 1), hill_map.graph.get_y_bounds());

    hill_map
}

fn map_djikstra<F>(map: &HillMap, source: Point, get_neighbors: F) -> (HashMap<Point, u128>, HashMap<Point, Point>)
    where F: Fn(&Point) -> Vec<Point> {
    let mut distances: HashMap<Point, u128> = HashMap::new();
    let mut paths: HashMap<Point, Point> = HashMap::new();

    let all_points = map.get_all_points();

    for point in &all_points {
        distances.insert(point.clone(), u128::MAX);
    }
    distances.insert(source, 0);

    // let mut queue: HashSet<Point> = HashSet::from_iter(all_points.iter().map(|point| point.clone()));
    let mut queue: PriorityQueue<Point, Reverse<u128>> = PriorityQueue::from_iter(all_points.iter().map(|point| (point.clone(), Reverse(distances.get(point).unwrap().clone()))));
    while !queue.is_empty() {
        let (current_point, distance_rev) = queue.pop().unwrap();
        let Reverse(distance) = distance_rev;
        if distance == u128::MAX {
            // no other nodes can reach me, I am the last node(s)
            continue;
        }
        let new_distance_to_neighbors = distance + 1;
        for neighbor_point in get_neighbors(&current_point) {
            let mut current_distance_to_neighbor = distances.get_mut(&neighbor_point).unwrap();
            if new_distance_to_neighbors < *current_distance_to_neighbor {
                *current_distance_to_neighbor = new_distance_to_neighbors;
                paths.insert(neighbor_point, current_point.clone());
                queue.change_priority(&neighbor_point, Reverse(new_distance_to_neighbors));
            }
        }
    }

    (distances, paths)
}

fn path_to_exit(map: &HillMap) -> u128 {
    let (distances, _) = map_djikstra(map, map.start, |current_point| map.get_accessible_neighbors(&current_point));
    distances[&map.goal]
}

fn path_from_exit(map: &HillMap) -> u128 {
    let (distances, _) = map_djikstra(map, map.goal, |current_point| map.get_neighbors_that_could_access(&current_point));
    distances.iter().filter_map(|(point, distance)| {
        if *map.graph.get_value(point).unwrap() != 0 {
            return None
        }
        Some(distance.clone())
    }).min().unwrap()
}

fn part1(input: &str) -> u128 {
    let map = parse_input(input);
    path_to_exit(&map)
}

fn part2(input: &str) -> u128 {
    let map = parse_input(input);
    path_from_exit(&map)
}

pub fn run() {
    println!("Day 12");
    let input = read_input(12);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    pub fn part1() {
        assert_eq!(31, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(29, super::part2(INPUT));
    }
}
