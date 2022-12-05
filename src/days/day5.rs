use itertools::Itertools;
use crate::common::read_input;
use regex::Regex;

struct MoveInstruction {
    count: u32,
    source: u32,
    dest: u32,
}

type CrateStack = Vec<char>;

fn parse_input(input: &str) -> (Vec<CrateStack>, Vec<MoveInstruction>) {
    let move_instruction_regex: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Could not create regex");
    let column_regex: Regex = Regex::new(r"\b\d+\b").expect("Could not create regex");

    let mut crates: Vec<CrateStack> = vec![];
    let mut instructions: Vec<MoveInstruction> = vec![];

    let mut parts: Vec<Vec<&str>> = input.split("\n\n").map(|part| part.split("\n").collect::<Vec<&str>>()).collect();
    let instructions_part: Vec<&str> = parts.pop().expect("Instructions part should be here");
    let mut crates_part: Vec<&str> = parts.pop().expect("Crates part should be here");

    let column_spots: &str = crates_part.pop().expect("No last spot");
    let column_count = column_regex.find_iter(column_spots).count();

    for _ in 0..column_count {
        crates.push(vec![]);
    }

    crates_part.reverse();
    for crate_line in crates_part {
        for column in 0..column_count {
            let column_char_index = (column * 4) + 1;
            let column_char_opt = crate_line.chars().nth(column_char_index);
            match column_char_opt {
                None => continue,
                Some(column_char) => {
                    if column_char != ' ' {
                        crates[column].push(column_char);
                    }
                }
            }
        }
    }

    for instruction_line in instructions_part {
        let captures = move_instruction_regex.captures(instruction_line).expect("No capture on instruction");
        let count: u32 = captures.get(1).expect("No capture").as_str().parse::<u32>().expect("Not a number");
        let source: u32 = captures.get(2).expect("No capture").as_str().parse::<u32>().expect("Not a number") - 1;
        let dest: u32 = captures.get(3).expect("No capture").as_str().parse::<u32>().expect("Not a number") - 1;

        instructions.push(MoveInstruction {
            count,
            source,
            dest,
        });
    }

    (crates, instructions)
}

fn part1(input: &str) -> String {
    let (mut crates, mut instructions) = parse_input(input);

    for instruction in instructions {
        for _ in 0..instruction.count {
            let source_item = crates[instruction.source as usize].pop().expect("No source crate");
            crates[instruction.dest as usize].push(source_item);
        }
    }

    crates.iter().filter_map(|stack| stack.last()).join("")
}

fn part2(input: &str) -> String {
    let (mut crates, mut instructions) = parse_input(input);

    for instruction in instructions {
        let mut source_items: CrateStack = vec![];

        for _ in 0..instruction.count {
            let source_item = crates[instruction.source as usize].pop().expect("No source crate");
            source_items.push(source_item);
        }

        for _ in 0..instruction.count {
            let source_item = source_items.pop().expect("No source item");
            crates[instruction.dest as usize].push(source_item);
        }
    }

    crates.iter().filter_map(|stack| stack.last()).join("")
}

pub fn run() {
    println!("Day 5");
    let input = read_input(5);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    pub fn part1() {
        assert_eq!("CMZ", super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!("MCD", super::part2(INPUT));
    }
}