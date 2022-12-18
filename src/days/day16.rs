use std::cmp::{max, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use itertools::{all, Itertools};
use priority_queue::PriorityQueue;
use regex::Regex;
use crate::common::movement::{Bounds, Point};
use crate::common::read_input;

struct ValveData<'a> {
    name: &'a str,
    flow_rate: u128,
    tunnels: Vec<&'a str>
}

fn parse_input(input: &str) -> Vec<ValveData> {
    let valve_regex = Regex::new(r"Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>\d+); .+? (?P<tunnels>(?:[A-Z]{2}(?:, )?)+)").unwrap();

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

fn dijkstra_step_count<'a>(valves: &'a HashMap<&'a str, &'a ValveData<'a>>, start_valve_name: &'a str) -> HashMap<&'a str, u128> {
    let mut distances: HashMap<&str, u128> = HashMap::from_iter(valves.keys().into_iter().map(|&valve_name| (valve_name, u128::MAX)));
    distances.insert(start_valve_name, 0);

    let mut queue: PriorityQueue<&str, Reverse<u128>> = PriorityQueue::new();
    for (&valve_name, &valve_data) in valves {
        queue.push(valve_name, Reverse(*distances.get(valve_name).unwrap()));
    }

    while !queue.is_empty() {
        let (current_name, Reverse(distance)) = queue.pop().unwrap();
        let current_data = *valves.get(current_name).unwrap();
        let current_neighbor_distance = distance + 1;

        for &neighbor_name in &current_data.tunnels {
            let existing_neighbor_distance = *distances.get(neighbor_name).unwrap();

            if current_neighbor_distance < existing_neighbor_distance {
                distances.insert(neighbor_name, current_neighbor_distance);
                queue.change_priority(neighbor_name, Reverse(current_neighbor_distance));
            }
        }
    }

    distances
}

struct PressureSearchData<'a> {
    all_valve_distances: &'a HashMap<&'a str, HashMap<&'a str, u128>>,
    valves: &'a HashMap<&'a str, &'a ValveData<'a>>,
    open_valves: HashSet<&'a str>,
    minutes_remaining: u128,
    current_valve_name: &'a str,
}

fn open_next_valve(data: PressureSearchData) -> u128 {
    let PressureSearchData { all_valve_distances, valves, open_valves, minutes_remaining, current_valve_name } = data;

    let current_pressure_per_minute: u128 = valves.iter().filter_map(|(&valve_name, &valve)| {
        if open_valves.contains(valve_name) {
            Some(valve.flow_rate)
        } else {
            None
        }
    }).sum();

    let unopened_valves: Vec<&ValveData> = valves.iter().filter_map(|(&valve_name, &valve)| {
        if valve.flow_rate == 0 || open_valves.contains(valve_name) {
            None
        } else {
            Some(valve)
        }
    }).collect();

    if unopened_valves.is_empty() {
        return minutes_remaining * current_pressure_per_minute;
    }

    let valve_distances = all_valve_distances.get(current_valve_name).unwrap();

    let mut max_flow_found = 0;
    for next_valve_to_open in unopened_valves {
        let minutes_spent_opening_valve = valve_distances.get(next_valve_to_open.name).unwrap() + 1;
        if minutes_spent_opening_valve > minutes_remaining {
            continue;
        }
        let pressure_released_while_moving = current_pressure_per_minute * minutes_spent_opening_valve;
        let mut open_valves_with_next = open_valves.clone();
        open_valves_with_next.insert(next_valve_to_open.name);
        // Takes 1 minute to open the valve, so subtract 1 extra
        max_flow_found = max(max_flow_found, pressure_released_while_moving + open_next_valve(PressureSearchData {
            all_valve_distances,
            valves,
            open_valves: open_valves_with_next,
            minutes_remaining: minutes_remaining - minutes_spent_opening_valve,
            current_valve_name: next_valve_to_open.name
        }));
    }

    max_flow_found
}

fn part1(input: &str) -> u128 {
    let valves = parse_input(input);
    let valves_by_name = HashMap::from_iter(valves.iter().map(|valve| (valve.name, valve)));

    let mut all_valve_distances: HashMap<&str, HashMap<&str, u128>> = HashMap::new();
    for valve in &valves {
        all_valve_distances.insert(valve.name, dijkstra_step_count(&valves_by_name, valve.name));
    }

    open_next_valve(PressureSearchData {
        all_valve_distances: &all_valve_distances,
        open_valves: HashSet::new(),
        valves: &valves_by_name,
        current_valve_name: "AA",
        minutes_remaining: 30
    })
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
        assert_eq!(1651, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(56000011, super::part2(INPUT));
    }
}
