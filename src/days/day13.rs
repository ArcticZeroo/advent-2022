use std::cmp::Ordering;
use itertools::Itertools;
use regex::Regex;
use crate::common::{read_input};
use crate::vm::{VirtualMachine, Instruction};

enum ListItem {
    Value(u128),
    List(Vec<ListItem>)
}

type ListPair = (ListItem, ListItem);

fn parse_list(input: &str) -> ListItem {
    let number_regex = Regex::new(r"\D").unwrap();
    let mut items = vec![];
    let mut list_stack: Vec<Vec<ListItem>> = vec![vec![]];
    let mut i = 0;
    while i < input.len() {
        match input.chars().nth(i).unwrap() {
            '[' => list_stack.push(vec![]),
            ']' => {
                let child_list = list_stack.pop().unwrap();
                if list_stack.is_empty() {
                    return ListItem::List(items);
                } else {
                    list_stack.last_mut().unwrap().push(ListItem::List(child_list));
                }
            },
            ',' => continue,
            _ => {
                let end_position = number_regex.find(&input[i..]).unwrap().end() - 1;
                println!("{}", &input[i..i+end_position]);
                let value = input[i..i+end_position].parse().expect("Could not parse value");
                list_stack.last_mut().unwrap().push(ListItem::Value(value));
                i += end_position;
            }
        }
        i += 1;
    }
    panic!("List did not close");
}

fn parse_pairs(input: &str) -> Vec<ListPair> {
    input.split("\n\n").map(|group| {
        let (left, right) = group.split_once("\n").unwrap();
        (parse_list(left), parse_list(right))
    }).collect()
}

fn value_as_list(value: &u128) -> ListItem {
    return ListItem::List(vec![ListItem::Value(value.clone())]);
}

fn is_in_order(left: &ListItem, right: &ListItem) -> bool {
    if let (ListItem::Value(left_value), ListItem::Value(right_value)) = (&left, &right) {
        return left_value < right_value;
    }

    if let (ListItem::List(left_items), ListItem::List(right_items)) = (left, right) {
        return match left_items.len().cmp(&right_items.len()) {
            Ordering::Greater => false,
            Ordering::Less => true,
            Ordering::Equal => {
                (0..left_items.len()).all(|i| is_in_order(&left_items[i], &right_items[i]))
            }
        };
    } else {
        if let ListItem::Value(left_value) = left {
            let left_list = value_as_list(left_value);
            return is_in_order(&left_list, right);
        } else if let ListItem::Value(right_value) = right {
            let right_list = value_as_list(right_value);
            return is_in_order(left, &right_list);
        }
    }

    panic!();
}

fn part1(input: &Vec<ListPair>) -> usize {
    input.iter().enumerate().filter_map(|(i, pair)| {
        let (left, right) = pair;
        if is_in_order(left, right) {
            Some(i)
        } else {
            None
        }
    }).sum::<usize>()
}

fn part2(input: &Vec<ListPair>) -> i128 {
    0
}

pub fn run() {
    println!("Day 13");
    let input = read_input(13);
    let pairs = parse_pairs(input.as_str());
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    pub fn part1() {
        let pairs = parse_pairs(INPUT);
        assert_eq!(13, super::part1(&pairs));
    }

    #[test]
    pub fn part2() {
        let pairs = parse_pairs(INPUT);
        assert_eq!(0, super::part2(&pairs));
    }
}
