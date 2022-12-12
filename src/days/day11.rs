use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

use crate::common::read_input;

enum Operation {
    Add,
    Multiply
}

enum Identifier {
    Old,
    Value(u128)
}

impl Identifier {
    pub fn value(&self, old: &u128) -> u128 {
        match self {
            Identifier::Old => *old,
            Identifier::Value(value) => *value
        }
    }
}

type MonkeyEquation = (Identifier, Operation, Identifier);

fn parse_identifier(value: &str) -> Identifier {
    if value == "old" {
        return Identifier::Old;
    }
    return Identifier::Value(value.parse().unwrap());
}

fn parse_equation(equation: &str) -> MonkeyEquation {
    let pieces: Vec<&str> = equation.split(" ").collect();
    let left = parse_identifier(pieces[0]);
    let operation = match pieces[1] {
        "*" => Operation::Multiply,
        "+" => Operation::Add,
        _ => panic!("bad op str")
    };
    let right = parse_identifier(pieces[2]);
    return (left, operation, right);
}

struct Monkey {
    items: VecDeque<u128>,
    is_bored: bool,
    operation: MonkeyEquation,
    test_amount: u128,
    true_pass_id: usize,
    false_pass_id: usize,
    inspected_items: usize,
}

impl Monkey {
    pub fn parse(lines: Vec<&str>, is_bored: bool) -> Monkey {
        let starting_items_raw = lines[1];
        let operation_raw = lines[2];
        let test = lines[3];
        let if_true = lines[4];
        let if_false = lines[5];
        let (_, starting_item_values) = starting_items_raw.split_once("items: ").unwrap();
        let starting_items: Vec<u128> = starting_item_values.split(", ").map(|item| u128::from(item.parse::<u128>().unwrap())).collect();
        let (_, true_monkey_id_str) = if_true.split_once("monkey ").unwrap();
        let (_, false_monkey_id_str) = if_false.split_once("monkey ").unwrap();
        let (_, operation) = operation_raw.split_once("new = ").unwrap();
        let (_, divisible_by_str) = test.split_once("by ").unwrap();
        Monkey {
            is_bored,
            items: VecDeque::from(starting_items),
            operation: parse_equation(operation),
            true_pass_id: true_monkey_id_str.parse().unwrap(),
            false_pass_id: false_monkey_id_str.parse().unwrap(),
            test_amount: divisible_by_str.parse().unwrap(),
            inspected_items: 0
        }
    }
}

fn do_operation((left, operation, right): &MonkeyEquation, value: &u128) -> u128 {
    match operation {
        Operation::Add => left.value(value) + right.value(value),
        Operation::Multiply => left.value(value) * right.value(value)
    }
}

fn do_monkey_diff(monkey: &mut Monkey, mod_amount: u128) -> HashMap<usize, Vec<u128>> {
    let mut diff: HashMap<usize, Vec<u128>> = HashMap::new();
    while !monkey.items.is_empty() {
        monkey.inspected_items += 1;
        let mut item = monkey.items.pop_front().unwrap();
        item = do_operation(&monkey.operation, &item);
        if monkey.is_bored {
            item /= u128::from(3 as u8);
        }
        item %= mod_amount;
        if item % monkey.test_amount == 0 {
            let true_pass_id = monkey.true_pass_id;
            diff.entry(true_pass_id).or_insert(vec![]).push(item);
        } else {
            let false_pass_id = monkey.false_pass_id;
            diff.entry(false_pass_id).or_insert(vec![]).push(item);
        }
    }
    diff
}

fn do_round(monkeys: &mut Vec<Monkey>, mod_amount: u128) {
    for i in 0..monkeys.len() {
        let item_diff = do_monkey_diff(&mut monkeys[i], mod_amount);
        for (monkey_id, items) in item_diff {
            monkeys[monkey_id].items.extend(items);
        }
    }
}

fn do_rounds(input: &str, is_bored: bool, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|group| Monkey::parse(group.split("\n").collect(), is_bored)).collect();
    let unique_divisors: HashSet<u128> = monkeys.iter().map(|monkey| monkey.test_amount).collect();
    let mod_amount: u128 = unique_divisors.iter().fold(1, |a, b| a * b) as u128;
    for _ in 0..rounds {
        do_round(&mut monkeys, mod_amount);
    }
    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));
    monkeys[0].inspected_items * monkeys[1].inspected_items
}

fn part1(input: &str) -> usize {
    do_rounds(input, true, 20)
}

fn part2(input: &str) -> usize {
    do_rounds(input, false, 10_000)
}

pub fn run() {
    println!("Day 11");
    let input = read_input(11);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    pub fn part1() {
        assert_eq!(10605, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(2713310158, super::part2(INPUT));
    }
}