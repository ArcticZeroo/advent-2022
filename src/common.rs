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