use std::fmt;
use std::fmt::Formatter;
use crate::common::read_input;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Move::Rock => "Rock",
            Move::Paper => "Paper",
            Move::Scissors => "Scissors"
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Outcome::Draw => "Draw",
            Outcome::Win => "Win",
            Outcome::Loss => "Loss"
        })
    }
}

fn move_value(move_type: Move) -> i32 {
    match move_type {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn outcome_value(outcome_type: Outcome) -> i32 {
    match outcome_type {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    }
}

fn move_from_guide(guide_value: char) -> Move {
    match guide_value {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!("Illegal guide value")
    }
}

fn outcome_from_guide(guide_value: char) -> Outcome {
    match guide_value {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Illegal guide value")
    }
}

fn get_beaten_by(move_type: Move) -> Move {
    match move_type {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock
    }
}

fn your_outcome(opponent_move: Move, your_move: Move) -> Outcome {
    if opponent_move == your_move {
        return Outcome::Draw;
    }

    if get_beaten_by(opponent_move) == your_move {
        return Outcome::Win;
    }

    Outcome::Loss
}

fn fight_score(opponent_move: Move, your_move: Move) -> i32 {
    let fight_outcome = your_outcome(opponent_move.clone(), your_move.clone());
    outcome_value(fight_outcome) + move_value(your_move.clone())
}

fn part1(input: &str) -> i32 {
    input.split("\n")
        .map(|line| {
            let opponent_move = move_from_guide(line.chars().nth(0).expect("Char missing"));
            let your_move = move_from_guide(line.chars().nth(2).expect("Char missing"));
            fight_score(opponent_move, your_move)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input.split("\n")
        .map(|line| {
            let opponent_move = move_from_guide(line.chars().nth(0).expect("Char missing"));
            let desired_outcome = outcome_from_guide(line.chars().nth(2).expect("Char missing"));
            let your_move = match desired_outcome {
                Outcome::Draw => opponent_move,
                Outcome::Win => get_beaten_by(opponent_move),
                Outcome::Loss => get_beaten_by(get_beaten_by(opponent_move))
            };
            outcome_value(desired_outcome) + move_value(your_move)
        })
        .sum()
}

pub fn run() {
    println!("Day 2");

    let input = read_input(2);

    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    pub fn part1() {
        assert_eq!(15, super::part1(INPUT))
    }

    #[test]
    pub fn part2() {
        assert_eq!(12, super::part2(INPUT))
    }
}