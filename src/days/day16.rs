use std::collections::{HashMap, HashSet};
use itertools::max;
use regex::Regex;
use crate::common::movement::{Bounds, Point};
use crate::common::read_input;

struct ValveData<'a> {
    name: &'a str,
    flow_rate: u128,
    tunnels: Vec<&'a str>
}

fn parse_input(input: &str) -> Vec<ValveData> {
    let valve_regex = Regex::new(r"Valve (?<name>[A-Z]{2}) has flow rate=(?<rate>\d+); .+? (?<tunnels>(?:[A-Z]{2}(?:, )?)+)").unwrap();

    valve_regex.captures_iter(input)
        .map(|capture| {
            let name = capture.name("name").unwrap().as_str();
            let flow_rate: u128 = capture.name("rate").unwrap().as_str().parse().unwrap();
            let tunnels: Vec<&str> = capture.name("tunnels").unwrap().as_str().split(", ").collect();

            ValveData {
                name,
                flow_rate,
                tunnels
            }
        })
        .collect()
}

fn max_flow_this_minute(valves: &HashMap<&str, ValveData>, open_valves: HashSet<&str>, minutes_remaining: u128, current_valve: &str) -> u128 {
    let current_valve_data = valves.get(current_valve).unwrap();

    let pressure_released_this_minute = open_valves.iter().map(|&valve| valves.get(valve).unwrap().flow_rate).sum();

    let max_flow_options = vec![];

    if minutes_remaining > 0 {

    }
    0
}

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
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    pub fn part1() {
        assert_eq!(26, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(56000011, super::part2(INPUT));
    }
}
