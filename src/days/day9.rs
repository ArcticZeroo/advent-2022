use std::cmp::{max, min};
use std::collections::HashSet;
use itertools::Itertools;
use crate::common::read_input;
use crate::common::movement::{GridDirection, Point};

struct MoveInstruction {
    direction: GridDirection,
    count: usize,
}

fn parse_instructions(input: &str) -> Vec<MoveInstruction> {
    input.split("\n")
        .map(|line| {
            let (direction, count) = line.split_once(" ").expect("Could not split");
            MoveInstruction {
                direction: match direction {
                    "R" => GridDirection::Right,
                    "L" => GridDirection::Left,
                    "U" => GridDirection::Up,
                    "D" => GridDirection::Down,
                    _ => panic!("Illegal direction {}", direction)
                },
                count: count.parse().expect("Could not parse count"),
            }
        })
        .collect()
}

fn render_grid(visited_positions: &HashSet<Point>, rope: &Vec<Point>) -> String {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for position in visited_positions {
        min_x = min(min_x, position.x);
        max_x = max(max_x, position.x);
        min_y = min(min_y, position.y);
        max_y = max(max_y, position.y);
    }
    (min_y - 1..=max_y + 1)
        .map(|y| (min_x - 1..=max_x + 1)
            .map(|x| -> String {
                let current = Point { x, y};
                if rope.iter().all(|point| current == *point) {
                    "+".to_string()
                } else if let Some(position) = rope.iter().position(|point| current == *point) {
                    (rope.len() - position).to_string()
                } else if current == Point::zero() {
                    "s".to_string()
                } else if visited_positions.contains(&Point { x, y }) {
                    "#".to_string()
                } else {
                    ".".to_string()
                }
            }).join("")
        ).join("\n")
}

fn move_knots(remaining_rope: &mut [Point]) {
    if remaining_rope.len() <= 1 {
        return
    }

    let head = remaining_rope[0];
    let knot = &mut remaining_rope[1];

    // println!("Moving knot at {:?} relative to head {:?}", knot, head);

    let distance = head.subtract(knot).magnitude();
    if distance < 2.into() {
        // println!("Distance is not far enough ({})", distance);
        return
    }

    // println!("Moving knot because it is too far, distance: {}", distance);
    if head.y == knot.y {
        // println!("x axis is shared, moving along x-axis");
        if head.x > knot.x {
            knot.x += 1;
        } else {
            knot.x -= 1;
        }
    } else if head.x == knot.x {
        // println!("y axis is shared, moving along y-axis");
        if head.y > knot.y {
            knot.y += 1;
        } else {
            knot.y -= 1;
        }
    } else {
        if head.x > knot.x {
            knot.x += 1;
        } else {
            knot.x -= 1;
        }

        if head.y > knot.y {
            knot.y += 1;
        } else {
            knot.y -= 1;
        }
    }

    move_knots(&mut remaining_rope[1..]);
}

fn move_head(rope: &mut Vec<Point>, direction: GridDirection, count: usize, out_positions: &mut HashSet<Point>) {
    // println!("===");
    // println!("{:?} {}", direction, count);
    // println!("Starting positions: head={:?}, tail={:?}", out_rope.head, out_rope.tail);
    // println!("Starting grid:");
    // println!("\n{}\n", render_grid(out_positions, &rope));

    for _ in 0..count {
        // println!("Moving");
        // println!("Current positions before move: head={:?}, tail={:?}", out_rope.head, out_rope.tail);
        rope[0].move_in_dir(direction.clone());
        // println!("Current positions after move: head={:?}, tail={:?}", out_rope.head, out_rope.tail);

        move_knots(rope);
        out_positions.insert(rope.last().expect("No last point").clone());

        // println!("\n{}\n", render_grid(out_positions, &rope));
    }
}

fn simulate_rope(input: &str, rope_size: usize) -> usize {
    let instructions = parse_instructions(input);

    let mut rope = vec![Point::zero(); rope_size];

    let mut visited_positions: HashSet<Point> = HashSet::new();
    visited_positions.insert(Point::zero());

    for instruction in instructions {
        move_head(&mut rope, instruction.direction, instruction.count, &mut visited_positions);
    }

    visited_positions.len()
}

fn part1(input: &str) -> usize {
    simulate_rope(input, 2)
}

fn part2(input: &str) -> usize {
    simulate_rope(input, 10)
}

pub fn run() {
    println!("Day 9");
    let input = read_input(9);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, super::part2(INPUT));
    }
}
