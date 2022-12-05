use std::fmt::{Formatter, write};
use crate::common;

#[derive(Clone, Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        Range {
            start,
            end,
        }
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
            || (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

struct Day4 {
    pairs: Vec<(Range, Range)>,
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .split("\n")
        .map(|line| {
            let pair_ranges = line
                .split(",")
                .map(|range_str| {
                    let range_values: Vec<i32> = range_str
                        .split("-")
                        .map(|value| value.parse::<i32>().expect("Could not parse range value"))
                        .collect();

                    Range::new(range_values[0], range_values[1])
                })
                .collect::<Vec<Range>>();
            (pair_ranges[0].clone(), pair_ranges[1].clone())
        })
        .collect()
}

impl Day4 {
    fn new(input: &str) -> Day4 {
        Day4 {
            pairs: parse_input(input)
        }
    }

    pub fn part1(&self) -> usize {
        self.pairs.iter().filter(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0)).count()
    }

    pub fn part2(&self) -> usize {
        self.pairs.iter().filter(|pair| pair.0.overlaps(&pair.1)).count()
    }
}

pub fn run() {
    println!("Day 4");
    let input = common::read_input(4);
    let day = Day4::new(input.as_str());
    println!("Part 1: {}", day.part1());
    println!("Part 2: {}", day.part2());
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let day = Day4::new(input);

        assert_eq!(2, day.part1());
    }

    #[test]
    pub fn part2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let day = Day4::new(input);

        assert_eq!(4, day.part2());
    }
}