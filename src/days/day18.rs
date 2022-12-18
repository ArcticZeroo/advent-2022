use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use itertools::max;
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
    pub fn get_axis_value(&self, axis: Axis3D) -> i128 {
        match axis {
            Axis3D::X => self.x,
            Axis3D::Y => self.y,
            Axis3D::Z => self.z
        }
    }

    pub fn get_moved_in_axis(&self, axis: Axis3D, amount: i128) -> Point3D {
        match axis {
            Axis3D::X => Point3D {
                x: self.x + amount,
                y: self.y,
                z: self.z,
            },
            Axis3D::Y => Point3D {
                x: self.x,
                y: self.y + amount,
                z: self.z,
            },
            Axis3D::Z => Point3D {
                x: self.x,
                y: self.y,
                z: self.z + amount,
            },
        }
    }

    pub fn get_with_axis_value(&self, axis: Axis3D, value: i128) -> Point3D {
        match axis {
            Axis3D::X => Point3D {
                x: value,
                y: self.y,
                z: self.z,
            },
            Axis3D::Y => Point3D {
                x: self.x,
                y: value,
                z: self.z,
            },
            Axis3D::Z => Point3D {
                x: self.x,
                y: self.y,
                z: value,
            },
        }
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
    let axis_movements = [-1, 1];
    let points: HashSet<Point3D> = HashSet::from_iter(parse_input(input));
    points
        .iter()
        .map(|point|
            Axis3D::all()
                .iter()
                .map(|&axis| axis_movements.iter().filter(|&&movement| !points.contains(&point.get_moved_in_axis(axis, movement))).count() as u128)
                .sum::<u128>()
        )
        .sum()
}

fn part2(input: &str) -> u128 {
    let points: HashSet<Point3D> = HashSet::from_iter(parse_input(input));
    let mut bounds_by_axis: HashMap<Axis3D, Bounds> = HashMap::new();

    for point in &points {
        for axis in Axis3D::all() {
            // let axis_entry = points_by_axis.entry(axis).or_insert_with(|| HashMap::new());
            // let value_entry = axis_entry.entry(point.get_axis_value(axis)).or_insert_with(|| HashSet::new());
            // value_entry.insert(point);
            bounds_by_axis
                .entry(axis)
                .or_insert_with(|| Bounds::new(0, 0))
                .update(point.get_axis_value(axis));
        }
    }

    points
        .iter()
        .map(|point| {
            println!("\nChecking for point {:?}", point);
            let mut exterior_faces = 0;
            for axis in Axis3D::all() {
                let axis_bounds = bounds_by_axis.get(&axis).unwrap();

                println!("Checking on axis {:?}", axis);
                println!("Axis bounds: {:?}", axis_bounds);

                let mut min_bounds = axis_bounds.min..point.get_axis_value(axis);
                let mut max_bounds = point.get_axis_value(axis) + 1..=axis_bounds.max;

                println!("Is face open on negative {:?} axis? {}", axis, !points.contains(&point.get_moved_in_axis(axis, -1)));

                if min_bounds.all(|axis_value| !points.contains(&point.get_with_axis_value(axis, axis_value))) {
                    println!("Negative {:?} axis is clear", axis);
                    exterior_faces += 1;
                } else {
                    println!("Negative {:?} axis is not clear", axis);
                }

                println!("Is face open on positive {:?} axis? {}", axis, !points.contains(&point.get_moved_in_axis(axis, 1)));

                if max_bounds.all(|axis_value| !points.contains(&point.get_with_axis_value(axis, axis_value))) {
                    println!("Positive {:?} axis is clear", axis);
                    exterior_faces += 1;
                } else {
                    println!("Positive {:?} axis is not clear", axis);
                }
            }
            exterior_faces
        })
        .sum()
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
