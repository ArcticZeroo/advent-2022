use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::common::read_input;
use regex::Regex;

type DirectoryTree<'a> = HashMap<&'a str, DirectoryItem<'a>>;

struct DirectoryItem<'a> {
    size: Option<u128>,
    name: &'a str,
    is_directory: bool,
    children: Option<DirectoryTree<'a>>,
    parent: Option<&'a mut DirectoryItem<'a>>,
}

impl DirectoryItem<'_> {
    pub fn new_directory<'a>(name: &'a str, parent: Option<&'a mut DirectoryItem<'a>>) -> DirectoryItem<'a> {
        DirectoryItem {
            size: None,
            is_directory: true,
            children: Some(DirectoryTree::new()),
            name,
            parent,
        }
    }

    pub fn new_item<'a>(name: &'a str, size: u128, parent: Option<&'a mut DirectoryItem<'a>>) -> DirectoryItem<'a> {
        DirectoryItem {
            is_directory: false,
            children: None,
            size: Some(size),
            name,
            parent,
        }
    }

    fn directory_size(&self) -> u128 {
        if !self.is_directory {
            panic!("Illegal usage of directory size");
        }

        let mut total_size: u128 = 0;
        for child in self.children.as_ref().expect("No children for directory").values() {
            if child.is_directory {
                total_size += child.directory_size();
            } else {
                total_size += child.size.expect("File should have size");
            }
        }
        total_size
    }
}

fn parse_root(input: &str) -> DirectoryItem {
    let command_regex: Regex = Regex::new(r"^(\w+)(.+?)").expect("Could not parse regex");

    let mut filesystem_root = DirectoryItem::new_directory("/", None /*parent*/);
    let mut current_directory = &mut filesystem_root;

    let mut raw_commands = input.split("$ ");

    for all_command_output in raw_commands.skip(1) {
        let command_lines: Vec<&str> = all_command_output.split_ascii_whitespace().collect();

        let main_command_line = command_lines.get(0).expect("Missing first line");

        let captures = command_regex.captures(main_command_line).expect("Could not parse command!");

        let command_name = captures.get(1).expect("Could not get command name").as_str();
        let args = captures.get(1);

        let output_lines = &command_lines[1..];

        match command_name {
            "cd" => {
                let directory = args.expect("Could not get args").as_str();
                match directory {
                    "/" => current_directory = &mut filesystem_root,
                    ".." => current_directory = current_directory.parent.expect("No parent, but we tried to go up"),
                    _ => current_directory = &mut current_directory.children.expect("No children").get(directory).expect("Child is missing from directory")
                }
            }
            "ls" => {
                for directory_output_line in output_lines {
                    let mut children = current_directory.children.as_ref().expect("Children should exist, this is a directory");
                    let (prefix, name) = directory_output_line.split_once(" ").expect("Could not split");
                    if prefix == "dir" {
                        children.insert(name, DirectoryItem::new_directory(name, Some(current_directory)));
                    } else {
                        let size: u128 = prefix.parse().expect("Could not parse size");
                        children.insert(name, DirectoryItem::new_item(name, size, Some(current_directory)));
                    }
                }
            }
            _ => {
                panic!("Unknown command {}", command_name)
            }
        }
    }

    filesystem_root
}

fn find_small_directories<'a>(node: &'a DirectoryItem<'a>, directories: &mut Vec<(&'a DirectoryItem<'a>, u128)>) {
    if !node.is_directory {
        return;
    }

    let directory_size = node.directory_size();
    if directory_size <= 10_000 {
        directories.push((node, directory_size));
    }

    for child in node.children.as_ref().unwrap().values() {
        find_small_directories(child, directories);
    }
}

fn part1(input: &str) -> u128 {
    let root = parse_root(input);
    let mut small_directories: Vec<(&DirectoryItem, u128)> = vec![];
    find_small_directories(&root, &mut small_directories);
    small_directories.iter().map(|(_, size)| size.clone()).sum()
}

fn part2(input: &str) -> i128 {
    0
}

pub fn run() {
    println!("Day 7");
    let input = read_input(7);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    pub fn part1() {
        assert_eq!(95437, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        // assert_eq!(19, super::part2(INPUT));
    }
}
