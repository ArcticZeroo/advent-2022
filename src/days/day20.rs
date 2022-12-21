use std::collections::VecDeque;
use itertools::Itertools;
use crate::common::{read_input, wrap_value};

fn parse_input(input: &str) -> VecDeque<i128> {
    input.split("\n").map(|line| line.parse::<i128>().unwrap()).collect()
}

struct Node {
    value: i128,
    original_index: usize,
}

fn mix_list(items: &mut VecDeque<Node>) {
    let mut current_items_index = 0;
    for current_index_to_mix in 0..items.len() {
        while items[current_items_index].original_index != current_index_to_mix {
            current_items_index = wrap_value((current_items_index + 1) as i128, items.len() as i128) as usize;
        }
        let node = items.remove(current_items_index).unwrap();
        items.insert(wrap_value((current_items_index as i128 + node.value as i128), items.len() as i128) as usize, node);
    }
}

fn find_grove_coordinates(items: &VecDeque<Node>) -> i128 {
    let zero_index = items.iter().position(|node| node.value == 0).expect("Zero is not in the list");
    items[(zero_index + 1000) % items.len()].value + items[(zero_index + 2000) % items.len()].value + items[(zero_index + 3000) % items.len()].value
}

fn part1(input: &str) -> i128 {
    let mut list: VecDeque<Node> = parse_input(input).iter().enumerate().map(|(i, &value)| Node {
        value,
        original_index: i,
    }).collect();
    mix_list(&mut list);
    find_grove_coordinates(&list)
}

const DECRYPTION_KEY: i128 = 811589153;

fn part2(input: &str) -> i128 {
    let mut list: VecDeque<Node> = parse_input(input).iter().enumerate().map(|(i, &value)| Node {
        value: value * DECRYPTION_KEY,
        original_index: i,
    }).collect();
    for _ in 0..10 {
        mix_list(&mut list);
    }
    find_grove_coordinates(&list)
}

pub fn run() {
    println!("Day 20");
    let input = read_input(20);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    pub fn part1() {
        assert_eq!(3, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(1623178306, super::part2(INPUT));
    }
}
