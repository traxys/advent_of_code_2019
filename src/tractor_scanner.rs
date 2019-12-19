use crate::intcode::{parse_intcode, IntcodeComputer};
use std::collections::HashMap;

fn scan_point(x: usize, y: usize, code: &[i64]) -> bool {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(x as i64);
    computer.add_input(y as i64);
    computer.run();
    *computer.output().last().unwrap() == 1
}

#[aoc(day19, part1)]
fn scan_points(scanner_code: &[i64]) -> usize {
    (0..50)
        .map(|x| {
            (0..50)
                .map(|y| scan_point(x, y, scanner_code) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn beam_x_bounds(y: usize, scanner_code: &[i64]) -> (usize, usize) {
    let mut x_start = y;
    if scan_point(x_start, y, scanner_code) {
        while scan_point(x_start, y, scanner_code) {
            x_start -= 1;
        }
        x_start += 1;
    } else {
        while !scan_point(x_start, y, scanner_code) {
            x_start += 1;
        }
    }
    let mut x_end = x_start;
    while scan_point(x_end, y, scanner_code) {
        x_end += 1;
    }
    (x_start, x_end - 1)
}
fn beam_y_bounds(x: usize, y: usize, scanner_code: &[i64]) -> (usize, usize) {
    let mut y_end = y;
    while scan_point(x, y_end, scanner_code) {
        y_end += 1;
    }
    (y, y_end - 1)
}

fn find_a_square(mut start: usize, size: usize, scanner_code: &[i64]) -> (usize, usize) {
    loop {
        start += 1;
        let (x_start, x_end) = beam_x_bounds(start, scanner_code);
        if x_end - x_start < size * 2 {
            continue;
        }
        let (y_start, y_end) = beam_y_bounds(x_end, start, scanner_code);
        if y_end - y_start < size * 2 {
            continue;
        } else {
            let (x_start_bot, _) = beam_x_bounds(y_start + 100, scanner_code);
            if x_start_bot < x_end && x_end - x_start_bot >= size {
                break (x_start_bot, start);
            }
        }
    }
}

fn square_in_beam(x: usize, y: usize, size: usize, scanner_code: &[i64]) -> bool {
    scan_point(x, y, scanner_code)
        && scan_point(x + size - 1, y, scanner_code)
        && scan_point(x + size - 1, y + size - 1, scanner_code)
        && scan_point(x, y + size - 1, scanner_code)
}

fn smallest_fit(mut x: usize, mut y: usize, size: usize, scanner_code: &[i64]) -> (usize, usize) {
    loop {
        let mut has_done = false;
        if square_in_beam(x - 1, y, size, scanner_code) {
            x -= 1;
            has_done = true;
        }
        if square_in_beam(x, y - 1, size, scanner_code) {
            y -= 1;
            has_done = true;
        }
        if square_in_beam(x - 1, y - 1, size, scanner_code) {
            y -= 1;
            x -= 1;
            has_done = true;
        }
        if !has_done {
            break (x, y);
        }
    }
}

#[aoc(day19, part2)]
fn find_distance(scanner_code: &[i64]) -> usize {
    let size = 100;
    let start = 250 * 4;
    let (x, y) = find_a_square(start, size, scanner_code);
    let (sx, sy) = smallest_fit(x, y, size, scanner_code);
    println!("{}, {}", sx, sy);

    10000 * sx + sy
}
