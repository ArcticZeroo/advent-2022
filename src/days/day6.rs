use itertools::Itertools;
use crate::common::read_input;

const START_OF_PACKET_SIZE: usize = 4;
const START_OF_MESSAGE_SIZE: usize = 14;

fn find_start_packet(input: &str, packet_size: usize) -> Option<usize> {
    for (i, window) in input.chars().collect::<Vec<char>>().windows(packet_size).enumerate() {
        if window.iter().all_unique() {
            return Some(i + packet_size);
        }
    }
    return None;
}

fn part1(input: &str) -> i128 {
    find_start_packet(input, START_OF_PACKET_SIZE).expect("Start packet not found") as i128
}

fn part2(input: &str) -> i128 {
    find_start_packet(input, START_OF_MESSAGE_SIZE).expect("Start packet not found") as i128
}

pub fn run() {
    println!("Day 6");
    let input = read_input(6);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        assert_eq!(7, super::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, super::part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, super::part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, super::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, super::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    pub fn part2() {
        assert_eq!(19, super::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, super::part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, super::part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, super::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, super::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
