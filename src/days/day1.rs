use crate::common;

struct Day1 {
    inventories: Vec<i32>
}

fn parse_input(input: &str) -> Vec<i32> {
    let mut inventories = input
        .trim()
        .split("\n\n")
        .map(|inventory| inventory.split("\n").map(|value| value.parse::<i32>().expect("Could not parse to int")).sum::<i32>())
        .collect::<Vec<i32>>();
    inventories.sort_by(|a, b| b.cmp(a));
    inventories
}


impl Day1 {
    fn new(input: &str) -> Day1 {
        Day1 {
            inventories: parse_input(input)
        }
    }

    pub fn part1(&self) -> i32 {
        self.inventories[0]
    }

    pub fn part2(&self) -> i32 {
        self.inventories[0..3].iter().sum::<i32>()
    }
}

pub fn run() {
    println!("Day 1");
    let input = common::read_input(1);
    let day = Day1::new(input.as_str());
    println!("Part 1: {}", day.part1());
    println!("Part 2: {}", day.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let mut day = Day1::new(input);
        assert_eq!(24000, day.part1());
    }
}