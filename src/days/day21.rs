use std::collections::HashMap;
use crate::common::{BinaryOperation, read_input};

enum MonkeyJob<'a> {
    Yell(u128),
    Math(&'a str, BinaryOperation, &'a str)
}

fn do_operation(left: u128, operation: BinaryOperation, right: u128) -> u128 {
    match operation {
        BinaryOperation::Add => left + right,
        BinaryOperation::Subtract => left - right,
        BinaryOperation::Multiply => left * right,
        BinaryOperation::Divide => left / right
    }
}

fn resolve_job(monkeys: &HashMap<&str, MonkeyJob>, job: &MonkeyJob) -> u128 {
    match job {
        MonkeyJob::Yell(value) => *value,
        MonkeyJob::Math(left_monkey_name, operation, right_monkey_name) => {
            let left_monkey_job = monkeys.get(left_monkey_name).unwrap();
            let right_monkey_job = monkeys.get(right_monkey_name).unwrap();
            do_operation(
                resolve_job(monkeys, left_monkey_job),
                *operation,
                resolve_job(monkeys, right_monkey_job)
            )
        }
        _ => panic!()
    }
}

fn parse_input(input: &str) -> HashMap<&str, MonkeyJob> {
    input.lines().map(|line| {
        let (name, job_str) = line.split_once(": ").unwrap();
        if let Ok(value) = job_str.parse() {
            (name, MonkeyJob::Yell(value))
        } else {
            let mut parts = job_str.split_ascii_whitespace();
            let left_monkey_name = parts.next().unwrap();
            let operation = BinaryOperation::from_str(parts.next().unwrap());
            let right_monkey_name = parts.next().unwrap();
            (name, MonkeyJob::Math(left_monkey_name, operation, right_monkey_name))
        }
    }).collect()
}

fn part1(input: &str) -> u128 {
    let monkeys = parse_input(input);
    resolve_job(&monkeys, monkeys.get("root").unwrap())
}

fn part2(input: &str) -> u128 {
    0
}

pub fn run() {
    println!("Day 21");
    let input = read_input(21);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    pub fn part1() {
        assert_eq!(152, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(1623178306, super::part2(INPUT));
    }
}
