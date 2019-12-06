#[macro_use]
extern crate aoc_runner_derive;

use std::collections::HashSet;

mod intcode;
mod intersectin_wires;

// use rayon::prelude::*;

fn fuel_for_mass(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn total_fuel(mass: i64) -> i64 {
    let mut current_mass = mass;
    let mut total_fuel_needed = 0;
    while fuel_for_mass(current_mass) > 0 {
        total_fuel_needed += fuel_for_mass(current_mass);
        current_mass = fuel_for_mass(current_mass);
    }
    total_fuel_needed
}

#[aoc(day1, part1)]
pub fn fuel_of_module(input: &str) -> i64 {
    input
        .lines()
        .map(|mass| fuel_for_mass(mass.parse().unwrap()))
        .sum()
}
#[aoc(day1, part2)]
pub fn fuel_of_fuels(input: &str) -> i64 {
    input
        .lines()
        .map(|mass| total_fuel(mass.parse().unwrap()))
        .sum()
}

#[aoc_generator(day4)]
pub fn get_range(range: &str) -> (Vec<u8>, Vec<u8>) {
    let mut bounds = range
        .split("-")
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap() as u8).collect());
    (bounds.next().unwrap(), bounds.next().unwrap())
}

fn generate_passwords(first_digit: u8, end_digit: u8, len: usize) -> HashSet<Vec<u8>> {
    let mut possibilities = HashSet::new();
    for i in first_digit..=end_digit {
        possibilities.insert(vec![i]);
    }
    for _ in 1..len {
        let mut new_possibilites = HashSet::new();
        for possibility in possibilities {
            for x in *possibility.last().unwrap()..=9 {
                let mut new_possibility = possibility.clone();
                new_possibility.push(x);
                new_possibilites.insert(new_possibility);
            }
        }
        possibilities = new_possibilites;
    }
    possibilities
}

#[aoc(day4, part1)]
pub fn count_passwords_in_range((start, end): &(Vec<u8>, Vec<u8>)) -> usize {
    generate_passwords(start[0], end[0], 6)
        .iter()
        .filter(|x| doubles(x).any(|_| true))
        .filter(|c| *c > start && *c < end)
        .count()
}

fn doubles(choice: &[u8]) -> impl Iterator<Item = (usize, u8)> + '_ {
    choice
        .iter()
        .enumerate()
        .zip(choice.iter().skip(1))
        .filter(|((_, x), y)| x == y)
        .map(|((i, x), _)| (i, *x))
}
fn consecutive_doubles(choice: &[u8]) -> impl Iterator<Item = u8> + '_ {
    doubles(choice)
        .zip(doubles(choice).skip(1))
        .filter(|((i, x), (j, y))| i + 1 == *j && x == y)
        .map(|((_, x), _)| x)
}
fn has_single_double(choice: &[u8]) -> bool {
    let doubles: HashSet<u8> = doubles(choice).map(|(_, x)| x).collect();
    let consecutive_double: HashSet<u8> = consecutive_doubles(choice).collect();
    doubles
        .symmetric_difference(&consecutive_double)
        .any(|_| true)
}

#[aoc(day4, part2)]
pub fn generate_better_passwords_in_range((start, end): &(Vec<u8>, Vec<u8>)) -> usize {
    generate_passwords(start[0], end[0], 6)
        .iter()
        .filter(|c| has_single_double(c))
        .filter(|c| *c > start && *c < end)
        .count()
}

use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn get_orbits(orbits: &str) -> HashMap<String, HashSet<String>> {
    let mut neighbours = HashMap::new();

    for (orbited, orbit) in orbits.lines().map(|x| {
        let mut foo = x.split(")");
        (foo.next().unwrap(), foo.next().unwrap())
    }) {
        neighbours
            .entry(orbited.to_owned())
            .or_insert_with(HashSet::new)
            .insert(orbit.to_owned());
        neighbours
            .entry(orbit.to_owned())
            .or_insert_with(HashSet::new)
            .insert(orbited.to_owned());
    }
    neighbours
}

fn get_lengths(
    graph: &HashMap<String, HashSet<String>>,
    start: String,
    stop_at: Option<&str>,
) -> HashMap<String, u64> {
    let mut lens = HashMap::new();
    let mut current_set = HashSet::new();
    current_set.insert(start.clone());
    lens.insert(start, 0);
    let mut curr_len = 0;

    let mut visited = HashSet::new();

    loop {
        let mut new_set = HashSet::new();
        for item in current_set {
            if !visited.contains(&item) {
                if let Some(childs) = graph.get(&item) {
                    for child in childs {
                        if !visited.contains(&item) {
                            new_set.insert(child.to_owned());
                        }
                    }
                }
                visited.insert(item.clone());
                lens.insert(item, curr_len);
            }
        }
        if let Some(stop_at) = stop_at {
            if visited.contains(stop_at) {
                break;
            }
        }
        if new_set.is_empty() {
            break;
        }
        current_set = new_set;
        curr_len += 1;
    }
    lens
}

#[aoc(day6, part1)]
pub fn count_orbits(graph: &HashMap<String, HashSet<String>>) -> u64 {
    let lens = get_lengths(graph, "COM".to_owned(), None);
    lens.values().copied().sum()
}

#[aoc(day6, part2)]
pub fn hops_to_santa(graph: &HashMap<String, HashSet<String>>) -> u64 {
    let lens = get_lengths(graph, "YOU".to_owned(), Some("SAN"));
    lens.get("SAN").expect("santa not found") - 2
}

aoc_lib! { year = 2019 }
