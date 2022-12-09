use std::collections::HashSet;
use std::ops::Range;
use itertools::Itertools;
use crate::common::read_input;
use regex::Regex;

fn parse_forest(input: &str) -> Vec<Vec<u32>> {
    input.split("\n")
        .map(|line| line.chars()
            .map(|c| c as u32 - '0' as u32)
            .collect::<Vec<u32>>()
        ).collect()
}

fn is_visible(forest: &Vec<Vec<u32>>, tree_x: usize, tree_y: usize) -> bool {
    let tree_height = forest[tree_y][tree_x];
    // left
    (0..tree_x).all(|x| forest[tree_y][x] < tree_height)
        // right
        || (tree_x + 1..forest[0].len()).all(|x| forest[tree_y][x] < tree_height)
        // top
        || (0..tree_y).all(|y| forest[y][tree_x] < tree_height)
        // bottom
        || (tree_y + 1..forest.len()).all(|y| forest[y][tree_x] < tree_height)
}

fn part1(input: &str) -> usize {
    let forest = parse_forest(input);
    (0..forest.len())
        .map(|y| (0..forest[y].len()).filter(|x| is_visible(&forest, x.clone(), y)).count())
        .sum()
}

struct Point {
    x: usize,
    y: usize,
}

fn count_visible_trees<I>(forest: &Vec<Vec<u32>>, tree_position: &Point, trees_to_check: I) -> usize
    where I: Iterator<Item=Point>
{
    let Point { x: tree_x, y: tree_y } = tree_position;
    let tree_height = forest[*tree_y][*tree_x];
    let mut tree_count = 0;
    for point in trees_to_check {
        let Point { x, y } = point;
        tree_count += 1;
        if forest[y][x] >= tree_height {
            break;
        }
    }
    tree_count
}

fn scenic_score(forest: &Vec<Vec<u32>>, tree_position: Point) -> usize {
    let Point { x: tree_x, y: tree_y } = tree_position;

    if tree_x == 0 || tree_y == 0 || tree_y == forest.len() - 1 || tree_x == forest[0].len() - 1 {
        return 0;
    }

    let left = count_visible_trees(forest, &tree_position, (0..tree_x).rev().map(|x| Point { x, y: tree_y.clone() }));
    let right = count_visible_trees(forest, &tree_position, (tree_x + 1..forest[0].len()).map(|x| Point { x, y: tree_y.clone() }));
    let top = count_visible_trees(forest, &tree_position, (0..tree_y).rev().map(|y| Point { x: tree_x.clone(), y }));
    let bottom = count_visible_trees(forest, &tree_position, (tree_y + 1..forest.len()).map(|y| Point { x: tree_x.clone(), y }));
    return left * right * top * bottom;
}

fn part2(input: &str) -> usize {
    let forest = parse_forest(input);

    (0..forest.len())
        .map(|y| (0..forest[y].len()).map(|x| scenic_score(&forest, Point { x, y })).max().expect("No max found"))
        .max()
        .expect("No max found")
}

pub fn run() {
    println!("Day 6");
    let input = read_input(8);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    pub fn part1() {
        assert_eq!(21, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(8, super::part2(INPUT));
    }
}
