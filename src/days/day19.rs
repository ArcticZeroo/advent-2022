use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::RangeInclusive;
use itertools::{max, merge, min};
use regex::Regex;
use crate::common::movement::Bounds;
use crate::common::read_input;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl ResourceType {
    pub fn all() -> Vec<ResourceType> {
        vec![
            ResourceType::Ore,
            ResourceType::Clay,
            ResourceType::Obsidian,
            ResourceType::Geode,
        ]
    }

    pub fn from_str(value: &str) -> ResourceType {
        match value {
            "ore" => ResourceType::Ore,
            "clay" => ResourceType::Clay,
            "obsidian" => ResourceType::Obsidian,
            "geode" => ResourceType::Geode,
            _ => panic!()
        }
    }
}

type RecipeCost = HashMap<ResourceType, u128>;
type BlueprintRecipes = HashMap<ResourceType, RecipeCost>;

struct Blueprint {
    id: u128,
    recipes: BlueprintRecipes,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let blueprint_regex = Regex::new(r"^Blueprint (?P<id>\d+): (?P<recipes>.+)$").unwrap();
    let recipe_regex = Regex::new(r"Each (?P<type>\w+?) robot costs (?P<requirements>.+?).").unwrap();

    input.split("\n").map(|blueprint_line| {
        let blueprint_captures = blueprint_regex.captures(blueprint_line).unwrap();
        let id: u128 = blueprint_captures.name("get").unwrap().as_str().parse().unwrap();

        let mut recipes: BlueprintRecipes = BlueprintRecipes::new();

        let recipes_str = blueprint_captures.name("recipes").unwrap().as_str();
        for recipe_captures in recipe_regex.captures_iter(recipes_str) {
            let resource = ResourceType::from_str(recipe_captures.name("type").unwrap().as_str());
            let requirements_str = recipe_captures.name("type").unwrap().as_str();
            let mut requirements = RecipeCost::new();
            for requirement_str in requirements_str.split(" and ") {
                let (type_str, amount_str) = requirement_str.split_once(" ").unwrap();
                requirements.insert(ResourceType::from_str(type_str), amount_str.parse().unwrap());
            }

            recipes.insert(resource, requirements);
        }

        Blueprint {
            id,
            recipes,
        }
    }).collect()
}

struct SearchData<'a> {
    blueprint: &'a Blueprint,
    robots_owned: HashMap<ResourceType, u128>,
    resources_owned: HashMap<ResourceType, u128>,
    minutes_left: u128,
}

fn merge_resources(a: &HashMap<ResourceType, u128>, b: &HashMap<ResourceType, u128>) -> HashMap<ResourceType, u128> {
    let mut merged: HashMap<ResourceType, u128> = HashMap::new();
    for &resource_map in vec![a, b].iter() {
        for (resource, count) in resource_map {
            *merged.entry(*resource).or_insert(0) += count;
        }
    }
    merged
}

fn find_max_geodes(data: SearchData) -> u128 {
    let SearchData { blueprint, robots_owned, resources_owned, minutes_left } = data;

    if minutes_left == 0 {
        resources_owned.get(&ResourceType::Geode).unwrap();
    }

    let robots_that_can_be_purchased: Vec<ResourceType> = blueprint.recipes.iter().filter_map(|(robot_to_purchase_type, costs)| {
        if costs.iter().all(|(resource_to_use, resource_cost)| resources_owned.get(resource_to_use).unwrap() >= resource_cost) {
            Some(*robot_to_purchase_type)
        } else {
            None
        }
    }).collect();

    let mut max_geodes = 0;
    for possible_robot_type in robots_that_can_be_purchased {

    }
    max_geodes
}

fn part1(input: &str) -> u128 {
    0
}

fn part2(input: &str) -> u128 {
    0
}

pub fn run() {
    println!("Day 19");
    let input = read_input(19);
    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    pub fn part1() {
        assert_eq!(64, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(58, super::part2(INPUT));
    }
}
