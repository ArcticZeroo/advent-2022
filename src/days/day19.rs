use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::RangeInclusive;
use itertools::{Itertools, max, merge, min};
use regex::Regex;
use crate::common::movement::Bounds;
use crate::common::read_input;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
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
            _ => panic!("Invalid resource type {}", value)
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
    let recipe_regex = Regex::new(r"Each (?P<type>\w+?) robot costs (?P<requirements>.+?)[.]").unwrap();

    input.split("\n").map(|blueprint_line| {
        let blueprint_captures = blueprint_regex.captures(blueprint_line).unwrap();
        let id: u128 = blueprint_captures.name("id").unwrap().as_str().parse().unwrap();

        let mut recipes: BlueprintRecipes = BlueprintRecipes::new();

        let recipes_str = blueprint_captures.name("recipes").unwrap().as_str();
        for recipe_captures in recipe_regex.captures_iter(recipes_str) {
            let resource = ResourceType::from_str(recipe_captures.name("type").unwrap().as_str());
            let requirements_str = recipe_captures.name("requirements").unwrap().as_str();
            let mut requirements = RecipeCost::new();
            for requirement_str in requirements_str.split(" and ") {
                let (amount_str, type_str) = requirement_str.split_once(" ").unwrap();
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

fn purchase_resources(owned: &HashMap<ResourceType, u128>, requirements: &RecipeCost) -> HashMap<ResourceType, u128> {
    let mut merged: HashMap<ResourceType, u128> = owned.clone();
    for (resource_type, resource_cost) in requirements.iter() {
        *merged.entry(*resource_type).or_insert(0) -= resource_cost;
    }
    merged
}

fn empty_resource_inventory() -> HashMap<ResourceType, u128> {
    HashMap::from_iter(ResourceType::all().iter().map(|&resource_type| (resource_type, 0)))
}

fn purchase_and_find_next(data: &SearchData, resources_after_minute: &RecipeCost, possible_robot_type: ResourceType) -> u128 {
    let SearchData { blueprint, robots_owned, resources_owned: _, minutes_left } = data;
    let mut possible_robots_owned = robots_owned.clone();
    *possible_robots_owned.entry(possible_robot_type).or_insert(0) += 1;
    let possible_new_resources = purchase_resources(resources_after_minute, blueprint.recipes.get(&possible_robot_type).unwrap());
    find_max_geodes(SearchData {
        blueprint,
        robots_owned: possible_robots_owned,
        resources_owned: possible_new_resources,
        minutes_left: minutes_left - 1
    })
}

fn is_robot_needed(blueprint: &Blueprint, robots_owned: &HashMap<ResourceType, u128>, robot_type: ResourceType) -> bool {
    let robots_owned_of_type = robots_owned.get(&robot_type).unwrap();
    // we could need this robot if any recipe requires at least this many robots
    blueprint.recipes.iter().any(|(recipe_type, recipe_requirements)| {
        recipe_requirements.contains_key(&robot_type) && robots_owned_of_type < recipe_requirements.get(&robot_type).unwrap()
    })
}

fn find_max_geodes(data: SearchData) -> u128 {
    let SearchData { blueprint, robots_owned, resources_owned, minutes_left } = &data;

    if *minutes_left == 0 {
        println!("Inventory in this branch: {:?}", resources_owned.iter().map(|(resource_type, amount)| format!("{:?}={}", resource_type, amount)).join(", "));
        return *resources_owned.get(&ResourceType::Geode).unwrap();
    }

    let robots_that_can_be_purchased: HashSet<ResourceType> = blueprint.recipes.iter().filter_map(|(robot_to_purchase_type, costs)| {
        let can_afford = costs.iter().all(|(resource_to_use, resource_cost)| resources_owned.get(resource_to_use).unwrap() >= resource_cost);
        if !can_afford {
            return None
        }

        if !is_robot_needed(blueprint, &robots_owned, *robot_to_purchase_type) {
            return None
        }

        Some(*robot_to_purchase_type)
    }).collect();

    let resources_after_minute = merge_resources(&robots_owned, &resources_owned);

    if robots_that_can_be_purchased.contains(&ResourceType::Geode) {
        return purchase_and_find_next(&data, &resources_after_minute, ResourceType::Geode);
    } else if robots_that_can_be_purchased.contains(&ResourceType::Obsidian) {
        return purchase_and_find_next(&data, &resources_after_minute, ResourceType::Obsidian);
    }

    let mut max_geodes = 0;
    for possible_robot_type in robots_that_can_be_purchased {
        max_geodes = std::cmp::max(max_geodes, purchase_and_find_next(&data, &resources_after_minute, possible_robot_type));
    }

    // What if we don't take any action?
    // todo: take action if we would otherwise never be able to afford the geode bot
    max_geodes = std::cmp::max(max_geodes, find_max_geodes(SearchData {
        blueprint,
        robots_owned: robots_owned.clone(),
        resources_owned: resources_after_minute,
        minutes_left: minutes_left - 1
    }));

    max_geodes
}

const MINUTES_PART_1: u128 = 24;

fn part1(input: &str) -> u128 {
    let blueprints = parse_input(input);
    let mut max_geodes_by_blueprint: HashMap<u128, u128> = HashMap::new();
    for blueprint in blueprints {
        let mut default_robots_owned: HashMap<ResourceType, u128> = empty_resource_inventory();
        default_robots_owned.insert(ResourceType::Ore, 1);

        println!("Finding max geodes for blueprint id {}", blueprint.id);
        max_geodes_by_blueprint.insert(blueprint.id, find_max_geodes(SearchData {
            blueprint: &blueprint,
            robots_owned: default_robots_owned,
            resources_owned: empty_resource_inventory(),
            minutes_left: MINUTES_PART_1
        }));
    }
    println!("Max geodes by blueprint: {:?}", max_geodes_by_blueprint);
    max_geodes_by_blueprint.iter().map(|(id, geodes)| id * geodes).sum()
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
        assert_eq!(33, super::part1(INPUT));
    }

    #[test]
    pub fn part2() {
        assert_eq!(58, super::part2(INPUT));
    }
}
