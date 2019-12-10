pub use itertools::Itertools;
pub use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn get_range(range: &str) -> (Password, Password) {
    let mut bounds = range
        .split("-")
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap() as u8).collect());
    (bounds.next().unwrap(), bounds.next().unwrap())
}

type Password = arrayvec::ArrayVec<[u8; 6]>;

fn generate_passwords(first_digit: u8, end_digit: u8, len: usize) -> HashSet<Password> {
    let mut possibilities = HashSet::new();
    for i in first_digit..=end_digit {
        let mut pass = Password::new();
        unsafe { pass.push_unchecked(i) };
        possibilities.insert(pass);
    }
    for _ in 1..len {
        let mut new_possibilites = HashSet::new();
        for possibility in possibilities {
            for x in *possibility.last().unwrap()..=9 {
                let mut new_possibility = possibility.clone();
                unsafe { new_possibility.push_unchecked(x) };
                new_possibilites.insert(new_possibility);
            }
        }
        possibilities = new_possibilites;
    }
    possibilities
}

#[aoc(day4, part1)]
pub fn count_passwords_in_range((start, end): &(Password, Password)) -> usize {
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
pub fn generate_better_passwords_in_range((start, end): &(Password, Password)) -> usize {
    generate_passwords(start[0], end[0], 6)
        .iter()
        .filter(|c| has_single_double(c))
        .filter(|c| *c > start && *c < end)
        .count()
}
