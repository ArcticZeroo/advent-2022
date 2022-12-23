use std::cmp::max;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use crate::common::{BinaryOperation, read_input};

#[derive(Copy, Clone)]
enum MonkeyJob<'a> {
    Yell(i128),
    Math(&'a str, BinaryOperation, &'a str)
}

fn do_operation<T>(left: T, operation: BinaryOperation, right: T) -> T
where T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>
{
    match operation {
        BinaryOperation::Add => left + right,
        BinaryOperation::Subtract => left - right,
        BinaryOperation::Multiply => left * right,
        BinaryOperation::Divide => left / right
    }
}

fn resolve_job(monkeys: &HashMap<&str, MonkeyJob>, job: &MonkeyJob) -> i128 {
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

fn part1(input: &str) -> i128 {
    let monkeys = parse_input(input);
    resolve_job(&monkeys, monkeys.get("root").unwrap())
}

fn contains_monkey(monkeys: &HashMap<&str, MonkeyJob>, target_monkey: &str, current_monkey: &MonkeyJob) -> bool {
    match current_monkey {
        MonkeyJob::Yell(_) => false,
        MonkeyJob::Math(left_name, _, right_name) => {
            *left_name == target_monkey
                || *right_name == target_monkey
                || contains_monkey(monkeys, target_monkey, monkeys.get(left_name).unwrap())
                || contains_monkey(monkeys, target_monkey, monkeys.get(right_name).unwrap())
        }
    }
}

const HUMAN_MONKEY_NAME: &str = "humn";

fn check_does_increasing_x_increase_result(mut monkeys: HashMap<&str, MonkeyJob>, job: &MonkeyJob) -> bool {
    monkeys.insert(HUMAN_MONKEY_NAME, MonkeyJob::Yell(50));
    let a = resolve_job(&monkeys, job);
    monkeys.insert(HUMAN_MONKEY_NAME, MonkeyJob::Yell(100));
    let b = resolve_job(&monkeys, job);
    return a < b;
}

fn find_human_value(mut monkeys: HashMap<&str, MonkeyJob>, job_with_human: &MonkeyJob, job_without_human: &MonkeyJob) -> i128 {
    let without_human_value = resolve_job(&monkeys, job_without_human);
    let does_increasing_x_increase_result = check_does_increasing_x_increase_result(monkeys.clone(), job_with_human);
    let mut min = i128::MIN / 10_000;
    let mut max = i128::MAX / 10_000;
    loop {
        let current = ((max - min) / 2) + min;

        monkeys.insert(HUMAN_MONKEY_NAME, MonkeyJob::Yell(current));

        let with_human_value = resolve_job(&monkeys, job_with_human);

        if with_human_value == without_human_value {
            return current;
        }

        if with_human_value < without_human_value {
            if does_increasing_x_increase_result {
                min = current;
            } else {
                max = current;
            }
        } else {
            if does_increasing_x_increase_result {
                max = current;
            } else {
                min = current;
            }
        }

        if min == max {
            panic!("Could not find value");
        }
    }
}

fn part2(input: &str) -> i128 {
    let mut monkeys = parse_input(input);

    if let MonkeyJob::Math(root_left_name, _, root_right_name) = monkeys.get("root").unwrap() {
        let root_left_job = monkeys.get(root_left_name).unwrap();
        let root_right_job = monkeys.get(root_right_name).unwrap();

        let is_monkey_in_left = contains_monkey(&monkeys, HUMAN_MONKEY_NAME, root_left_job);

        if is_monkey_in_left {
            find_human_value(monkeys.clone(), root_left_job, root_right_job)
        } else {
            find_human_value(monkeys.clone(), root_right_job, root_left_job)
        }
    } else {
        panic!();
    }
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
        assert_eq!(301, super::part2(INPUT));
    }
}
