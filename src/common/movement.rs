use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::{RangeInclusive};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Bounds<T = i128> where T: Ord + Copy {
    pub min: T,
    pub max: T,
    _has_had_update: bool
}

impl<T> Bounds<T> where T: Ord + Copy {
    pub fn new(min: T, max: T) -> Bounds<T> {
        Bounds {
            min,
            max,
            _has_had_update: false
        }
    }

    pub fn update(&mut self, value: T) {
        if !self._has_had_update {
            self.min = value;
            self.max = value;
        } else {
            if value < self.min {
                self.min = value;
            } else if value > self.max {
                self.max = value;
            }
        }

        self._has_had_update = true;
    }

    pub fn to_range(&self) -> RangeInclusive<T> {
        self.min..=self.max
    }
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

    pub fn get_moved_in_dir(&self, direction: GridDirection) -> Point {
        let mut other = self.clone();
        other.move_in_dir(direction);
        other
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
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    pub fn manhattan_dist(&self, other: &Point) -> u128 {
        return ((self.x - other.x).abs() + (self.y - other.y).abs()) as u128
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GridDirection {
    Left,
    Right,
    Up,
    Down,
}

impl GridDirection {
    pub fn all() -> Vec<GridDirection> {
        vec![GridDirection::Left, GridDirection::Right, GridDirection::Up, GridDirection::Down]
    }
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

pub struct Grid<T> {
    _x_bounds: Bounds<i128>,
    _y_bounds: Bounds<i128>,
    _grid: HashMap<Point, T>,
}

impl<T: 'static> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid {
            _x_bounds: Bounds::new(0, 0),
            _y_bounds: Bounds::new(0, 0),
            _grid: HashMap::new(),
        }
    }

    pub fn visit(&mut self, point: Point, value: T) {
        self._grid.insert(point, value);
        self._x_bounds.update(point.x);
        self._y_bounds.update(point.y);
    }

    pub fn remove(&mut self, point: Point) {
        self._grid.remove(&point);
    }

    pub fn has_visited(&self, point: &Point) -> bool {
        self._grid.contains_key(point)
    }

    pub fn get_value(&self, point: &Point) -> Option<&T> {
        self._grid.get(point)
    }

    pub fn get_x_bounds(&self) -> Bounds {
        self._x_bounds.clone()
    }

    pub fn get_y_bounds(&self) -> Bounds {
        self._y_bounds.clone()
    }
}