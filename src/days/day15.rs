use std::collections::{HashMap, HashSet};
use itertools::max;
use regex::Regex;
use crate::common::movement::{Bounds, Point};
use crate::common::read_input;

fn parse_input(input: &str) -> HashMap<Point, Point> {
    let point_regex = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    input.split("\n")
        .map(|line| {
            let points: Vec<Point> = point_regex.captures_iter(line).map(|capture| Point { x: capture.get(1).unwrap().as_str().parse().unwrap(), y: capture.get(2).unwrap().as_str().parse().unwrap() }).collect();
            (points[0], points[1])
        })
        .collect()
}

fn part1(input: &str, y: i128) -> u128 {
    let beacons_by_sensor = parse_input(input);
    let sensor_radii: HashMap<Point, u128> = beacons_by_sensor.iter().map(|(sensor, beacon)| (sensor.clone(), sensor.manhattan_dist(beacon))).collect();
    let beacons: HashSet<Point> = beacons_by_sensor.values().map(|point| point.clone()).collect();
    let max_radius_for_line = *sensor_radii.iter().filter_map(|(sensor, radius)| {
        if (y - sensor.y).abs() > *radius as i128 {
            None
        } else {
            Some(radius)
        }
    }).max().unwrap() as i128;
    let mut x_bounds: Bounds<i128> = Bounds::new(0, 0);
    beacons_by_sensor.keys().for_each(|sensor| x_bounds.update(sensor.x));
    let mut positions_without_beacon = 0;
    println!("Checking beacon positions between {} and {}", x_bounds.min - max_radius_for_line, x_bounds.min + max_radius_for_line);
    for x in x_bounds.min - max_radius_for_line..=x_bounds.max + max_radius_for_line {
        let point = Point { x, y };
        let can_beacon_exist = sensor_radii.iter().all(|(beacon, &radius)| point.manhattan_dist(beacon) > radius);
        if !can_beacon_exist && !beacons.contains(&point) {
            positions_without_beacon += 1;
        }
    }
    positions_without_beacon
}

fn points_n_away(source_point: &Point, n: u128) -> Vec<Point> {
    let mut n_away_square: Vec<Point> = vec![];
    for i in 0..n {
        let x = i as i128;
        let y = (n - i) as i128;
        n_away_square.extend(vec![Point { x, y }, Point { x: -x, y }, Point { x: -x, y: -y }, Point { x, y: -y }]);
    }
    n_away_square.iter().map(|point| source_point.add(point)).collect()
}

fn part2(input: &str, max_coordinate: u128) -> u128 {
    let x_range = 0..=max_coordinate as i128;
    let y_range = 0..=max_coordinate as i128;
    let beacons_by_sensor = parse_input(input);
    let sensor_radii: HashMap<Point, u128> = beacons_by_sensor.iter().map(|(sensor, beacon)| (sensor.clone(), sensor.manhattan_dist(beacon))).collect();
    for (sensor, radius) in sensor_radii.iter() {
        for point in points_n_away(sensor, radius + 1) {
            if !x_range.contains(&point.x) || !y_range.contains(&point.y) {
                continue;
            }

            if sensor_radii.iter().all(|(other_sensor, other_radius)| point.manhattan_dist(other_sensor) > *other_radius) {
                return (point.x as u128 * 4_000_000) + point.y as u128;
            }
        }
    }
    panic!("Did not find a point that satisfies all conditions.");
}

pub fn run() {
    println!("Day 15");
    let input = read_input(15);
    println!("Part 1: {}", part1(input.as_str(), 2_000_000));
    println!("Part 2: {}", part2(input.as_str(), 4_000_000));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    pub fn part1() {
        assert_eq!(26, super::part1(INPUT, 10));
    }

    #[test]
    pub fn part2() {
        assert_eq!(56000011, super::part2(INPUT, 20));
    }
}
