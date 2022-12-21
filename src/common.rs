use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

pub mod movement;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("input/day{}.txt", day))
        .expect("Could not read input")
        .trim()
        .to_string()
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
