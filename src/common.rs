use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

impl Point {
    pub fn zero() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }

    pub fn move_in_dir(&mut self, direction: GridDirection) {
        match direction {
            GridDirection::Right => self.x += 1,
            GridDirection::Left => self.x -= 1,
            GridDirection::Up => self.y += 1,
            GridDirection::Down => self.y -= 1
        }
    }

    pub fn move_along_axis(&mut self, axis: Axis, count: i128) {
        match axis {
            Axis::Horizontal => self.x += count,
            Axis::Vertical => self.y += count
        }
    }

    pub fn get_coord_by_axis(&self, axis: Axis) -> i128 {
        match axis {
            Axis::Horizontal => self.x,
            Axis::Vertical => self.y
        }
    }

    pub fn subtract(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GridDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn from_dir(dir: GridDirection) -> Axis {
        match dir {
            GridDirection::Down | GridDirection::Up => Axis::Vertical,
            GridDirection::Left | GridDirection::Right => Axis::Horizontal
        }
    }

    pub fn opposite(&self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal
        }
    }
}