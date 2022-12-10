use std::collections::HashSet;
use crate::common::{intersect_to_set, read_input, string_to_set};
use itertools::Itertools;

fn priority(value: &char) -> u32 {
    if value.is_uppercase() {
        return 27 + (*value as u32 - 'A' as u32);
    }
    return 1 + (*value as u32 - 'a' as u32);
}

fn part1(input: &str) -> u32 {
    input.split("\n")
        .map(|line| {
            let midpoint = line.len() / 2;
            let a = string_to_set(&line[..midpoint]);
            let b = string_to_set(&line[midpoint..]);
            let invalid_char = a.intersection(&b).nth(0).expect("No char was in both sides");
            priority(invalid_char)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input.split("\n")
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let sets: Vec<HashSet<char>> = chunk.map(string_to_set).collect();
            let a = &sets[0];
            let b = &sets[1];
            let c = &sets[2];
            let common_intersection = intersect_to_set(&intersect_to_set(a, b), c);
            let common_char = common_intersection.iter().nth(0).expect("No common char");
            priority(common_char)
        })
        .sum()
}

pub fn run() {
    println!("Day 3");

    let input = read_input(3);

    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    pub fn part1() {
        assert_eq!(157, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(70, super::part2(INPUT));
    }
}