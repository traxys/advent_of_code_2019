mod minimalist_intcode;
mod computer;
pub use computer::{IntcodeComputer, IntcodeState};

#[aoc_generator(day11)]
#[aoc_generator(day9)]
#[aoc_generator(day7)]
#[aoc_generator(day5)]
pub fn parse_intcode(code: &str) -> Vec<i64> {
    code.split(",").map(|c| c.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn execute_better_intcode(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(1);
    computer.run();

    *computer.output().last().unwrap()
}

#[aoc(day5, part2)]
pub fn intcode_thermal_radiators(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(5);
    computer.run();

    *computer.output().last().unwrap()
}

use itertools::Itertools;

use std::cell::RefCell;
use std::collections::HashMap;
fn run_network(computers: &[RefCell<IntcodeComputer>], links: &HashMap<usize, Vec<usize>>) {
    loop {
        let mut all_finished = true;
        for (i, computer) in computers.iter().enumerate() {
            let new_state = computer.borrow_mut().step();
            all_finished &= new_state == IntcodeState::Finished;
            if let IntcodeState::Outputed = new_state {
                let new_input = *computer.borrow().output().last().unwrap();
                if let Some(linked) = links.get(&i) {
                    for linked in linked {
                        computers[*linked].borrow_mut().add_input(new_input);
                    }
                }
            }
        }
        if all_finished {
            break;
        }
    }
}

fn chain(first: usize, last: usize) -> HashMap<usize, Vec<usize>> {
    let mut links = HashMap::new();
    for x in first..last {
        links.insert(x, vec![x + 1]);
    }
    links
}

fn prepare_amps(computers: &[RefCell<IntcodeComputer>], phase_scale: &[u8]) {
    for (computer, phase) in computers.iter().zip(phase_scale) {
        computer.borrow_mut().add_input(*phase as i64)
    }
}
fn run_amps(phase_scale: &[u8], code: &[i64]) -> i64 {
    let computers: Vec<_> = (0..5)
        .map(|_| IntcodeComputer::new(Vec::from(code)))
        .map(RefCell::new)
        .collect();
    prepare_amps(&computers, phase_scale);
    computers[0].borrow_mut().add_input(0);

    run_network(&computers, &chain(0, 4));

    let x = *computers[4].borrow().output().last().unwrap();
    x
}

#[aoc(day7, part1)]
pub fn amplify_the_signal(code: &[i64]) -> i64 {
    (0..5)
        .permutations(5)
        .map(|c| run_amps(&c, code))
        .max()
        .expect("No permutation")
}

fn looped(start: usize, end: usize) -> HashMap<usize, Vec<usize>> {
    let mut chain = chain(start, end);
    chain.insert(end, vec![start]);
    chain
}

fn run_feedbacked_amps(phase_scale: &[u8], code: &[i64]) -> i64 {
    let computers: Vec<_> = (0..5)
        .map(|_| IntcodeComputer::new(Vec::from(code)))
        .map(RefCell::from)
        .collect();
    prepare_amps(&computers, phase_scale);
    computers[0].borrow_mut().add_input(0);

    run_network(&computers, &looped(0, 4));

    let x = *computers[4].borrow().output().last().unwrap();
    x
}

#[aoc(day7, part2)]
pub fn amplify_the_signal_with_feedback(code: &[i64]) -> i64 {
    (5..10)
        .permutations(5)
        .map(|c| run_feedbacked_amps(&c, code))
        .max()
        .expect("No permutation")
}

#[aoc(day9, part1)]
pub fn test_boost(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(1);
    computer.run();

    *computer.output().last().unwrap()
}
#[aoc(day9, part2)]
pub fn find_coordinates(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(2);
    computer.run();

    *computer.output().last().unwrap()
}
