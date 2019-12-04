#[macro_use]
extern crate aoc_runner_derive;

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

#[aoc_generator(day2)]
pub fn get_intcode(input: &str) -> Vec<u64> {
    input.split(",").map(|u| u.parse().unwrap()).collect()
}

fn run_intcode(code: &mut [u64]) -> Result<u64, String> {
    let mut position = 0;
    while position < code.len() {
        match code[position] {
            1 => {
                let source_a = code[position + 1] as usize;
                let source_b = code[position + 2] as usize;
                let dest = code[position + 3] as usize;
                code[dest] = code[source_a] + code[source_b];
            }
            2 => {
                let source_a = code[position + 1] as usize;
                let source_b = code[position + 2] as usize;
                let dest = code[position + 3] as usize;
                code[dest] = code[source_a] * code[source_b];
            }
            99 => return Ok(code[0]),
            i => return Err(format!("invalid opcode: {}", i)),
        }
        position += 4;
    }
    return Err("did not land on 99".to_owned());
}

#[aoc(day2, part1)]
pub fn execute_intcode(code: &[u64]) -> Result<u64, String> {
    let mut code = Vec::from(code);
    code[1] = 12;
    code[2] = 2;
    run_intcode(&mut code)
}

#[aoc(day2, part2)]
pub fn find_good_code(initial_memory: &[u64]) -> Result<u64, String> {
    let target_value = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut code = Vec::from(initial_memory);
            code[1] = noun;
            code[2] = verb;
            if run_intcode(&mut code)? == target_value {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No pair found".to_owned())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement {
    direction: Direction,
    length: i64,
}
type Path = Vec<Movement>;

#[aoc_generator(day3)]
pub fn get_paths(input: &str) -> (Path, Path) {
    let mut paths = input.lines().map(|line| {
        line.split(",")
            .map(|point| {
                let length = point[1..].parse().unwrap();
                let direction = if point.starts_with("R") {
                    Direction::Right
                } else if point.starts_with("D") {
                    Direction::Down
                } else if point.starts_with("U") {
                    Direction::Up
                } else if point.starts_with("L") {
                    Direction::Left
                } else {
                    panic!("Invalid direction: {}", point)
                };
                Movement { direction, length }
            })
            .collect()
    });
    (paths.next().unwrap(), paths.next().unwrap())
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

use std::collections::HashMap;

fn go_to(start: Point, movement: Movement) -> (Point, Box<dyn Iterator<Item = Point>>) {
    let mut end_point = start;
    match movement.direction {
        Direction::Up => {
            end_point.y += movement.length;
            (
                end_point,
                Box::new((0..=movement.length).map(move |i| Point {
                    x: start.x,
                    y: start.y + i,
                })),
            )
        }
        Direction::Down => {
            end_point.y -= movement.length;
            (
                end_point,
                Box::new((0..=movement.length).map(move |i| Point {
                    x: start.x,
                    y: start.y - i,
                })),
            )
        }
        Direction::Left => {
            end_point.x -= movement.length;
            (
                end_point,
                Box::new((0..=movement.length).map(move |i| Point {
                    x: start.x - i,
                    y: start.y,
                })),
            )
        }
        Direction::Right => {
            end_point.x += movement.length;
            (
                end_point,
                Box::new((0..=movement.length).map(move |i| Point {
                    x: start.x + i,
                    y: start.y,
                })),
            )
        }
    }
}

use std::collections::HashSet;
fn run_path(path: &Path) -> (HashSet<Point>, HashMap<Point, i64>) {
    let mut points = HashSet::new();
    let mut steps = HashMap::new();
    let mut current_point = Point { x: 0, y: 0 };
    steps.insert(current_point, 0);
    for movement in path {
        let (new_point, passed_points) = go_to(current_point, *movement);
        let start_steps = *steps
            .get(&current_point)
            .expect(&format!("Start point is unknown: {:?}", current_point));
        points.extend(passed_points.enumerate().map(|(i, p)| {
            let point_step = start_steps + i as i64;
            steps.entry(p).or_insert(point_step);
            p
        }));
        current_point = new_point;
    }
    (points, steps)
}

#[aoc(day3, part1)]
pub fn find_nearest_intersection((path1, path2): &(Path, Path)) -> i64 {
    let (mut passed1, _) = run_path(path1);
    passed1.remove(&Point { x: 0, y: 0 });
    let (passed2, _) = run_path(path2);
    passed1
        .intersection(&passed2)
        .map(|Point { x, y }| (x + y).abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn find_intersection_with_least_steps((path1, path2): &(Path, Path)) -> i64 {
    let (mut passed1, steps1) = run_path(path1);
    passed1.remove(&Point { x: 0, y: 0 });
    let (passed2, steps2) = run_path(path2);
    passed1
        .intersection(&passed2)
        .map(|p| *steps1.get(p).unwrap() + *steps2.get(p).unwrap())
        .min()
        .unwrap()
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

aoc_lib! { year = 2019 }
