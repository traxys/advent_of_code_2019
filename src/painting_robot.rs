use crate::intcode::{IntcodeState, IntcodeComputer, parse_intcode};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn advance(&self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Up => (0, 1),
            Direction::Left => (1, 0),
            Direction::Down => (0, -1),
            Direction::Right => (-1, 0),
        };
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn counter_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}
pub struct Robot {
    computer: IntcodeComputer,
    position: Point,
    direction: Direction,

    white: HashSet<Point>,
}

impl Robot {
    fn new(computer: IntcodeComputer) -> Self {
        Self {
            computer,
            position: Point{x: 0, y: 0},
            direction: Direction::Up,

            white: HashSet::new(),
        }
    }
    fn next_output(&mut self) -> Option<i64> {
        loop {
            match self.computer.step() {
                IntcodeState::Finished => break None,
                IntcodeState::NeedsInput => panic!("Why u need input"),
                IntcodeState::Ready => continue,
                IntcodeState::Outputed => break Some(*self.computer.output().last().expect("no output")),
            }
        }
    }
    fn step(&mut self) -> bool {
        let input = self.white.contains(&self.position) as i64;
        self.computer.add_input(input);
        let color = match self.next_output() {
            Some(0) => Color::Black,
            Some(1) => Color::White,
            Some(i) => panic!("Invalid color: {}", i),
            None => return false,
        };
        match color {
            Color::White => self.white.insert(self.position),
            Color::Black => self.white.remove(&self.position),
        };
        self.direction = match self.next_output() {
            Some(0) => self.direction.clockwise(),
            Some(1) => self.direction.counter_clockwise(),
            Some(i) => panic!("Invalid turning point: {}", i),
            None => return false,
        };
        self.position = self.position.advance(self.direction);
        true
    }
    fn run(&mut self) {
        while self.step() {}
    }
}

#[aoc(day11, part1)]
pub fn count_paint(input: &[i64]) -> usize {
    let computer = IntcodeComputer::new(Vec::from(input));
    let mut robot = Robot::new(computer);
    let mut been_at = HashSet::new();
    been_at.insert(robot.position);
    while robot.step() {
        been_at.insert(robot.position);
    }
    been_at.len()
}

fn draw_image(white: &HashSet<Point>) {
    let min_y = white.iter().map(|p| p.y).min().unwrap();
    let min_x = white.iter().map(|p| p.x).min().unwrap();

    let max_y = white.iter().map(|p| p.y).max().unwrap();
    let max_x = white.iter().map(|p| p.x).max().unwrap();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let pixel = if white.contains(&Point{x, y}) {
                yansi::Paint::new(" ").bg(yansi::Color::White)
            } else {
                yansi::Paint::new(" ")
            };
            print!("{}", pixel);
        }
        print!("\n");
    }
}

#[aoc(day11, part2)]
pub fn show_paint(input: &[i64]) -> &'static str {
    let computer = IntcodeComputer::new(Vec::from(input));
    let mut robot = Robot::new(computer);
    robot.white.insert(robot.position);
    robot.run();
    draw_image(&robot.white);
    ""
}
