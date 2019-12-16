#[macro_use]
extern crate aoc_runner_derive;

pub mod asteroid;
pub mod image;
pub mod intcode;
pub mod intersectin_wires;
pub mod orbits;
pub mod painting_robot;
pub mod password;
pub mod simulate_planets;
pub mod ball_breaker;
pub mod fuel_creation;
pub mod robot_exploration;
pub mod flawed_transmission;

/*use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;

#[global_allocator]
pub static A: CountingAllocator = CountingAllocator {
    total_time: RefCell::new(std::time::Duration::from_secs(0)),
};

pub struct CountingAllocator {
    total_time: RefCell<std::time::Duration>,
}

unsafe impl Sync for CountingAllocator {}

impl CountingAllocator {
    fn print_time(&self) {
        println!("{}", self.total_time.borrow().as_micros())
    }
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = std::time::Instant::now();
        let alloc = System.alloc(layout);
        *self.total_time.borrow_mut() += start.elapsed();
        alloc
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}*/

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

aoc_lib! { year = 2019 }
