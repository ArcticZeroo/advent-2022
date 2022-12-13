use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use itertools::{Itertools, min};
use regex::Regex;
use crate::common::{read_input};
use crate::vm::{VirtualMachine, Instruction};

#[derive(Debug, Eq, PartialEq)]
enum ListItem {
    Value(u128),
    List(Vec<ListItem>)
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ListItem::Value(value) => value.to_string(),
            ListItem::List(items) => format!("[{}]", items.iter().join(", "))
        })
    }
}

type ListPair = (ListItem, ListItem);

fn parse_list(input: &str) -> ListItem {
    let number_regex = Regex::new(r"\d+").unwrap();
    let mut list_stack: Vec<Vec<ListItem>> = vec![];
    let mut i = 0;
    while i < input.len() {
        match input.chars().nth(i).unwrap() {
            '[' => list_stack.push(vec![]),
            ']' => {
                let child_list = list_stack.pop().unwrap();
                if list_stack.is_empty() {
                    // the child we just popped was the parent
                    return ListItem::List(child_list);
                } else {
                    list_stack.last_mut().unwrap().push(ListItem::List(child_list));
                }
            },
            ',' => (),
            _ => {
                let end_position = number_regex.find(&input[i..]).unwrap().end();
                let value = input[i..i+end_position].parse().expect("Could not parse value");
                list_stack.last_mut().unwrap().push(ListItem::Value(value));
                i += end_position;
                continue;
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

fn get_left_ordering(left: &ListItem, right: &ListItem) -> Ordering {
    if let (ListItem::Value(left_value), ListItem::Value(right_value)) = (&left, &right) {
        return left_value.cmp(right_value);
    }

    if let (ListItem::List(left_items), ListItem::List(right_items)) = (left, right) {
        let shared_item_count = std::cmp::min(left_items.len(), right_items.len());
        for i in 0..shared_item_count {
             match get_left_ordering(&left_items[i], &right_items[i]) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => ()
            }
        }
        return left_items.len().cmp(&right_items.len());
    } else {
        if let ListItem::Value(left_value) = left {
            let left_list = value_as_list(left_value);
            return get_left_ordering(&left_list, right);
        } else if let ListItem::Value(right_value) = right {
            let right_list = value_as_list(right_value);
            return get_left_ordering(left, &right_list);
        }
    }

    panic!();
}

fn part1(input: &Vec<ListPair>) -> usize {
    input.iter().enumerate().filter_map(|(i, pair)| {
        let (left, right) = pair;
        if get_left_ordering(left, right) != Ordering::Greater {
            Some(i + 1)
        } else {
            None
        }
    }).sum::<usize>()
}

fn part2(input: &Vec<ListPair>) -> usize {
    let divider_packets = vec![
        ListItem::List(vec![ListItem::List(vec![ListItem::Value(2)])]),
        ListItem::List(vec![ListItem::List(vec![ListItem::Value(6)])])
    ];
    let sorted_packets: Vec<&ListItem> = input.iter().map(|(left, right)| vec![left, right]).flatten().chain(divider_packets.iter()).sorted_by(|&left, &right| get_left_ordering(left, right)).collect();
    let mut first_divider_index = 0;
    for i in 0..sorted_packets.len() {
        if let ListItem::List(top_list) = sorted_packets[i] {
            if top_list.len() != 1 {
                continue;
            }
            if let ListItem::List(child_list) = top_list.first().unwrap() {
                if child_list.len() != 1 {
                    continue;
                }
                if let (ListItem::Value(value)) = child_list.first().unwrap() {
                    match value {
                        2 => first_divider_index = i + 1,
                        6 => return first_divider_index * (i + 1),
                        _ => ()
                    }
                }
            }
        }
    }
    panic!("Could not find divider packets");
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
        assert_eq!(140, super::part2(&pairs));
    }
}
