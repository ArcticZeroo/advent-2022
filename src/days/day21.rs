use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use crate::common::{BinaryOperation, read_input};

enum MonkeyJob<'a> {
    Yell(i64),
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

fn resolve_job(monkeys: &HashMap<&str, MonkeyJob>, job: &MonkeyJob) -> i64 {
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

fn part1(input: &str) -> i64 {
    let monkeys = parse_input(input);
    resolve_job(&monkeys, monkeys.get("root").unwrap())
}

enum ResolvedValue {
    // In test + input, it is always humn (aka x) - other
    X(f64 /*multiplier aka left*x */, f64 /*right*/),
    Immediate(f64)
}

impl ResolvedValue {
    fn _do_x_operation(x_multiplier: f64, x_subtract: f64, operation: BinaryOperation, other_value: f64, is_x_on_left: bool) -> ResolvedValue {
        match operation {
            BinaryOperation::Add => ResolvedValue::X(x_multiplier, x_subtract - other_value),
            BinaryOperation::Subtract => ResolvedValue::X(x_multiplier, other_value + x_subtract),
            BinaryOperation::Multiply => ResolvedValue::X(x_multiplier * other_value as f64, x_subtract * other_value),
            BinaryOperation::Divide => {
                if is_x_on_left {
                    ResolvedValue::X(x_multiplier / other_value as f64, x_subtract / other_value)
                } else {
                    ResolvedValue::X(other_value as f64 / x_multiplier, other_value / x_subtract)
                }
            }
        }
    }

    pub fn do_operation(&self, operation: BinaryOperation, other: ResolvedValue) -> ResolvedValue {
        if let ResolvedValue::X(x_multiplier, x_subtract) = self {
            if let ResolvedValue::Immediate(other_value) = other {
                // (x - a) ??? b
                ResolvedValue::_do_x_operation(*x_multiplier, *x_subtract, operation, other_value, true /*is_x_on_left*/)
            } else {
                panic!();
            }
        } else if let ResolvedValue::X(x_multiplier, x_subtract) = other {
            if let ResolvedValue::Immediate(other_value) = self {
                // b ??? (x - a)
                ResolvedValue::_do_x_operation(x_multiplier, x_subtract, operation, *other_value, false /*is_x_on_left*/)
            } else {
                panic!();
            }
        } else {
            if let ResolvedValue::Immediate(self_value) = self {
                if let ResolvedValue::Immediate(other_value) = other {
                    ResolvedValue::Immediate(do_operation(*self_value, operation, other_value))
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }
    }
}

fn resolve_job_with_x(monkeys: &HashMap<&str, MonkeyJob>, job: &MonkeyJob) -> ResolvedValue {
    match job {
        MonkeyJob::Yell(value) => ResolvedValue::Immediate(*value as f64),
        MonkeyJob::Math(left_monkey_name, operation, right_monkey_name) => {
            let left_monkey_job = monkeys.get(left_monkey_name).unwrap();
            let right_monkey_job = monkeys.get(right_monkey_name).unwrap();

            if *left_monkey_name == "humn" {
                let right_resolved = resolve_job_with_x(monkeys, right_monkey_job);
                if let ResolvedValue::Immediate(right_value) = right_resolved {
                    return ResolvedValue::X(1.0, right_value)
                } else {
                    panic!();
                }
            }

            let left_resolved = resolve_job_with_x(monkeys, left_monkey_job);
            let right_resolved = resolve_job_with_x(monkeys, right_monkey_job);

            left_resolved.do_operation(*operation, right_resolved)
        }
        _ => panic!()
    }
}

fn find_inequality_value(x_multiplier: f64, x_subtract: f64, other_value: f64) -> f64 {
    (other_value + x_subtract) / x_multiplier
}

fn part2(input: &str) -> f64 {
    let monkeys = parse_input(input);

    if let MonkeyJob::Math(root_left_name, _, root_right_name) = monkeys.get("root").unwrap() {
        let root_left_job = monkeys.get(root_left_name).unwrap();
        let root_right_job = monkeys.get(root_right_name).unwrap();

        let root_left_resolved = resolve_job_with_x(&monkeys, root_left_job);
        let root_right_resolved = resolve_job_with_x(&monkeys, root_right_job);

        if let ResolvedValue::X(x_multiplier, x_subtract) = root_left_resolved {
            if let ResolvedValue::Immediate(right_value) = root_right_resolved {
                // (ax - b) = c
                find_inequality_value(x_multiplier, x_subtract, right_value)
            } else {
                panic!();
            }
        } else if let ResolvedValue::X(x_multiplier, x_subtract) = root_right_resolved {
            if let ResolvedValue::Immediate(left_value) = root_left_resolved {
                // c = (ax - b)
                find_inequality_value(x_multiplier, x_subtract, left_value)
            } else {
                panic!();
            }
        } else {
            panic!("x is missing");
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
        assert_eq!(301.0, super::part2(INPUT));
    }
}
