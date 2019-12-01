mod utils;
use std::io::prelude::*;

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

fn main() {
    let config = utils::input::get_config();
    let input = utils::input::open_input(&config);
    println!(
        "[PART 1] total fuel is: {}",
        input
            .lines()
            .map(|mass| fuel_for_mass(mass.unwrap().parse().unwrap()))
            .sum::<i64>()
    );

    let input = utils::input::open_input(&config);
    println!(
        "[PART 2] total fuel is: {}",
        input
            .lines()
            .map(|mass| total_fuel(mass.unwrap().parse().unwrap()))
            .sum::<i64>()
    );
}
