use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::RangeInclusive;
use itertools::{max, min};
use regex::Regex;
use crate::common::movement::Bounds;
use crate::common::read_input;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Axis3D {
    X,
    Y,
    Z,
}

impl Axis3D {
    pub fn all() -> Vec<Axis3D> {
        vec![Axis3D::X, Axis3D::Y, Axis3D::Z]
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point3D {
    x: i128,
    y: i128,
    z: i128,
}

impl Point3D {
    pub fn zero() -> Point3D {
        Point3D {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn get_axis_value(&self, axis: Axis3D) -> &i128 {
        match axis {
            Axis3D::X => &self.x,
            Axis3D::Y => &self.y,
            Axis3D::Z => &self.z
        }
    }

    pub fn set_axis_value(&mut self, axis: Axis3D, value: i128) {
        match axis {
            Axis3D::X => self.x = value,
            Axis3D::Y => self.y = value,
            Axis3D::Z => self.z = value
        }
    }

    pub fn get_moved_in_axis(&self, axis: Axis3D, amount: i128) -> Point3D {
        self.get_with_axis_value(axis, self.get_axis_value(axis) + amount)
    }

    pub fn get_with_axis_value(&self, axis: Axis3D, value: i128) -> Point3D {
        let mut other = self.clone();
        other.set_axis_value(axis, value);
        other
    }

    pub fn neighbors(&self) -> Vec<Point3D> {
        let axis_movements = [-1, 1];
        Axis3D::all()
            .iter()
            .flat_map(|&axis| axis_movements.iter().map(move |&movement| self.get_moved_in_axis(axis, movement)))
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Point3D> {
    input.split("\n").map(|line| {
        let coordinates: Vec<i128> = line.split(",").map(|part| part.parse::<i128>().unwrap()).collect();
        Point3D {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        }
    }).collect()
}

fn part1(input: &str) -> u128 {
    let points: HashSet<Point3D> = HashSet::from_iter(parse_input(input));
    points
        .iter()
        .map(|point|
            point.neighbors()
                .iter()
                .filter(|&neighbor| !points.contains(neighbor))
                .count() as u128
        )
        .sum()
}

fn is_in_bounds(point: &Point3D, ranges_by_axis: &HashMap<&Axis3D, RangeInclusive<i128>>) -> bool {
    Axis3D::all()
        .iter()
        .all(|axis| ranges_by_axis.get(axis).unwrap().contains(point.get_axis_value(*axis)))
}

fn count_open_faces(points: &HashSet<Point3D>) -> u128 {
    let mut bounds_by_axis: HashMap<Axis3D, Bounds> = HashMap::new();

    for point in points {
        for axis in Axis3D::all() {
            bounds_by_axis
                .entry(axis)
                .or_insert_with(|| Bounds::new(0, 0))
                .update(*point.get_axis_value(axis));
        }
    }

    let expanded_bounds_by_axis: HashMap<&Axis3D, Bounds> = HashMap::from_iter(bounds_by_axis.iter().map(|(axis, bounds)| (axis, Bounds::new(bounds.min - 1, bounds.max + 1))));
    let ranges_by_axis = HashMap::from_iter(expanded_bounds_by_axis.iter().map(|(&axis, bounds)| (axis, bounds.to_range())));

    let mut min_point = Point3D::zero();
    for axis in Axis3D::all() {
        min_point.set_axis_value(axis, expanded_bounds_by_axis.get(&axis).unwrap().min);
    }

    let mut open_faces = 0;
    let mut visited: HashSet<Point3D> = HashSet::new();
    let mut queue: VecDeque<Point3D> = VecDeque::new();

    queue.push_back(min_point);
    visited.insert(min_point);

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        for neighbor in next.neighbors() {
            if points.contains(&neighbor) {
                open_faces += 1;
            } else if !visited.contains(&neighbor) && is_in_bounds(&neighbor, &ranges_by_axis) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    open_faces
}

fn part2(input: &str) -> u128 {
    let points: HashSet<Point3D> = HashSet::from_iter(parse_input(input));
    count_open_faces(&points)
}

pub fn run() {
    println!("Day 18");
    let input = read_input(18);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    pub fn part1() {
        assert_eq!(64, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(58, super::part2(INPUT));
    }
}
