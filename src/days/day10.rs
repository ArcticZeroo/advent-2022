use itertools::Itertools;
use crate::common::{read_input};
use crate::vm::{VirtualMachine, Instruction};

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

fn part1(input: &str) -> i128 {
    let instructions = input.split("\n").map(|line| VirtualMachine::parse_instruction(line));
    let mut vm = VirtualMachine::new();
    let mut out: Vec<i128> = vec![];
    for instruction in instructions {
        vm.execute_instruction(instruction, |vm_ref| -> () {
            let cycle = vm_ref.get_cycle() as i128;
            if ((cycle - 20) % 40) == 0 {
                let x = vm_ref.get_register_value("x").expect("Register X is missing");
                out.push(cycle * x);
            }
        });
    }
    out.iter().sum()
}

fn part2(input: &str) -> String {
    let instructions = input.split("\n").map(|line| VirtualMachine::parse_instruction(line));
    let mut crt: Vec<Vec<char>> = vec![vec![]; CRT_HEIGHT];
    let mut vm = VirtualMachine::new();

    for instruction in instructions {
        vm.execute_instruction(instruction, |vm_ref| {
            let cycle = vm_ref.get_cycle();
            let draw_y = (cycle - 1) / CRT_WIDTH;
            let draw_x = ((cycle - 1) % CRT_WIDTH) as i128;
            let x_register = vm_ref.get_register_value("x").unwrap_or(1);
            let is_sprite_visible = (x_register - 1..=x_register + 1).contains(&draw_x);
            crt[draw_y].push(if is_sprite_visible { '#' } else { '.' });
        });
    }

    crt.iter().map(|line| line.iter().join("")).join("\n")
}

pub fn run() {
    println!("Day 10");
    let input = read_input(10);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2:\n{}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    pub fn part1() {
        assert_eq!(13140, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....", super::part2(INPUT));
    }
}
