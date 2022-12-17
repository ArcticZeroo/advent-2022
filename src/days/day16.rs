use std::collections::{HashMap, HashSet};
use itertools::max;
use regex::Regex;
use crate::common::movement::{Bounds, Point};
use crate::common::read_input;

fn part1(input: &str) -> u128 {
    0
}

fn part2(input: &str) -> u128 {
    0
}

pub fn run() {
    println!("Day 16");
    let input = read_input(16);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "";

    #[test]
    pub fn part1() {
        assert_eq!(26, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(56000011, super::part2(INPUT));
    }
}
