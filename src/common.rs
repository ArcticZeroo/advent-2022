use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Rem, Sub};
use crate::common::movement::Bounds;

pub mod movement;

fn read_input_trim_opt(day: i32, should_trim: bool) -> String {
    let input = fs::read_to_string(format!("input/day{}.txt", day))
        .expect("Could not read input");
    if should_trim {
        input.trim()
            .to_string()
    } else {
        input
    }
}

pub fn read_input(day: i32) -> String {
    read_input_trim_opt(day, true)
}

pub fn read_input_no_trim(day: i32) -> String {
    read_input_trim_opt(day, false)
}

pub fn intersect_to_set<T: Eq + Hash + Clone>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.intersection(&b).cloned().collect::<HashSet<T>>()
}

pub fn string_to_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

pub fn char_alphabet_position(value: char, is_lowercase: bool) -> u32 {
    value as u32 - if is_lowercase { 'a' as u32 } else { 'A' as u32 }
}

pub fn wrap_value(value: i128, wrap_around: i128) -> i128 {
    if value < 0 {
        value + (wrap_around * ((value.abs() / wrap_around) + 1))
    } else {
        value % wrap_around
    }
}

pub fn wrap_value_around_bounds(value: i128, bounds: &Bounds<i128>) -> i128 {
    if value > bounds.max {
        let diff_from_max = value - bounds.max;
        return bounds.min + (diff_from_max - 1);
    }

    if value < bounds.min {
        let diff_from_min = bounds.min - value;
        return bounds.max - (diff_from_min - 1);
    }

    value
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperation {
    pub fn from_str(value: &str) -> BinaryOperation {
        match value {
            "*" => BinaryOperation::Multiply,
            "/" => BinaryOperation::Divide,
            "+" => BinaryOperation::Add,
            "-" => BinaryOperation::Subtract,
            _ => panic!()
        }
    }
}
