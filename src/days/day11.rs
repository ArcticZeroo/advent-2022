use std::collections::{HashMap, VecDeque};
use eval::Expr;

struct Monkey<'a> {
    items: VecDeque<u128>,
    is_bored: bool,
    operation: &'a str,
    test_amount: u128,
    true_pass_id: usize,
    false_pass_id: usize,
    inspected_items: usize,
}

impl <'a> Monkey<'a> {
    pub fn parse(lines: Vec<&'a str>) -> Monkey<'a> {
        let starting_items_raw = lines[1];
        let operation_raw = lines[2];
        let test = lines[3];
        let if_true = lines[4];
        let if_false = lines[5];
        let (_, starting_item_values) = starting_items_raw.split_once("items: ").unwrap();
        let starting_items: Vec<u128> = starting_item_values.split(", ").map(|item| item.parse::<u128>().unwrap()).collect();
        let (_, true_monkey_id_str) = if_true.split_once("monkey ").unwrap();
        let (_, false_monkey_id_str) = if_false.split_once("monkey ").unwrap();
        let (_, operation) = operation_raw.split_once("new = ").unwrap();
        let (_, divisible_by_str) = test.split_once("by ").unwrap();
        Monkey {
            is_bored: true,
            items: VecDeque::from(starting_items),
            operation,
            true_pass_id: true_monkey_id_str.parse().unwrap(),
            false_pass_id: false_monkey_id_str.parse().unwrap(),
            test_amount: divisible_by_str.parse().unwrap(),
            inspected_items: 0
        }
    }
}

fn do_monkey_diff(monkey: &mut Monkey) -> HashMap<usize, Vec<u128>> {
    let mut diff: HashMap<usize, Vec<u128>> = HashMap::new();
    while !monkey.items.is_empty() {
        monkey.inspected_items += 1;
        let mut item = monkey.items.pop_front().unwrap();
        item = Expr::new(monkey.operation).value("old", item as u64).exec().unwrap().as_u64().unwrap() as u128;
        if monkey.is_bored {
            item /= 3;
        }
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

fn do_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let item_diff = do_monkey_diff(&mut monkeys[i]);
        for (monkey_id, items) in item_diff {
            monkeys[monkey_id].items.extend(items);
        }
    }
}

fn part1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|group| Monkey::parse(group.split("\n").collect())).collect();
    for _ in 0..20 {
        do_round(&mut monkeys);
    }
    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));
    monkeys[0].inspected_items * monkeys[1].inspected_items
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
}