use crate::intcode::{IntcodeComputer, IntcodeState, parse_intcode};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_int(value: i64) -> Tile {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            i => panic!("Invalid {}", i),
        }
    }
}

pub struct Game {
    screen: std::collections::HashMap<Point, Tile>,
    paddle: Point,
    score: usize,
    ball: Point,
    computer: IntcodeComputer,
}

enum Update {
    Draw(Point, Tile),
    Score(usize),
    Input,
}

impl Game {
    fn new(computer: IntcodeComputer) -> Self {
        Self {
            computer,
            ball: Point{x: 0, y: 0},
            paddle: Point{x: 0, y: 0},
            score: 0,
            screen: HashMap::new(),
        }
    }
    fn read_input(&mut self) -> Option<Update> {
        let x = loop {
            match self.computer.step() {
                IntcodeState::Ready => continue,
                IntcodeState::NeedsInput => return Some(Update::Input),
                IntcodeState::Finished => return None,
                IntcodeState::Outputed => break *self.computer.output().last().unwrap(),
            }
        };
        let y = loop {
            match self.computer.step() {
                IntcodeState::Ready => continue,
                IntcodeState::NeedsInput => return Some(Update::Input),
                IntcodeState::Finished => return None,
                IntcodeState::Outputed => break *self.computer.output().last().unwrap(),
            }
        };
        let tile = loop {
            match self.computer.step() {
                IntcodeState::Ready => continue,
                IntcodeState::NeedsInput => return Some(Update::Input),
                IntcodeState::Finished => return None,
                IntcodeState::Outputed => break *self.computer.output().last().unwrap(),
            }
        };
        if x == -1 && y == 0 {
            Some(Update::Score(tile as usize))
        } else {
            Some(Update::Draw(Point { x: x as usize, y: y as usize }, Tile::from_int(tile)))
        }
    }
    fn execute(&mut self) {
        loop {
            match self.read_input() {
                Some(Update::Draw(p, Tile::Empty)) => {
                    self.screen.remove(&p);
                }
                Some(Update::Draw(p, Tile::Ball)) => {
                    self.ball = p;
                }
                Some(Update::Draw(p, Tile::Paddle)) => {
                    self.paddle = p;
                }
                Some(Update::Draw(p, t)) => {
                    self.screen.insert(p, t);
                }
                Some(Update::Score(i)) => {
                    self.score = i;
                }
                Some(Update::Input) => {
                    self.input_joystick();
                }
                None => return,
            }
        }
    }
    fn input_joystick(&mut self) {
        let joystick_direction = match self.ball.x.cmp(&self.paddle.x) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        self.computer.add_input(joystick_direction);
    }
}

#[aoc(day13, part1)]
pub fn count_tiles(code: &[i64]) -> usize {
    let computer = IntcodeComputer::new(Vec::from(code));
    let mut game = Game::new(computer);
    game.execute();

    game.screen
        .iter()
        .filter(|(_, t)| **t == Tile::Block)
        .count()
}

#[aoc(day13, part2)]
pub fn play_the_game(code: &[i64]) -> usize {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.arbitrary_set(0, 2);
    let mut game = Game::new(computer);
    game.execute();

    game.score
}
