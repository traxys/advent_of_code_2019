use crate::intcode::{parse_intcode, IntcodeComputer, IntcodeState};
use std::collections::{HashMap, HashSet};

#[aoc(day15, part1)]
fn how_far_is_system(robot_code: &[i64]) -> usize {
    let computer = IntcodeComputer::new(Vec::from(robot_code));
    let mut robot = Robot::new(computer);
    loop {
        match robot.explore() {
            Some(State::Tank) => break,
            None => robot.panic("No tank"),
            _ => continue,
        }
    }
    let tank = robot.position;
    robot.reinit_computer(IntcodeComputer::new(Vec::from(robot_code)));
    let resp = robot.go_to_by_explored(tank);
    resp
}
#[aoc(day15, part2)]
fn explore_all(robot_code: &[i64]) -> usize {
    let computer = IntcodeComputer::new(Vec::from(robot_code));
    let mut robot = Robot::new(computer);
    while let Some(_) = robot.explore() {}

    let known = robot.known;
    let empty_count = known.values().filter(|s| **s != State::Wall).count();
    let tank = *known.iter().find(|(_, s)| **s == State::Tank).map(|(p, _)| p).unwrap();

    let mut oxygen = HashSet::new();
    oxygen.insert(tank);
    let mut current = oxygen.clone();
    let mut i = 0;
    while oxygen.len() != empty_count {
        let mut spread = HashSet::new();
        
        for point in current {
            for neighbour in &point.neighbours() {
                if !oxygen.contains(neighbour) {
                    match known.get(neighbour) {
                        Some(&State::Empty) | Some(&State::Tank) => {
                            spread.insert(*neighbour);
                            oxygen.insert(*neighbour);
                        }
                        _ => continue,
                    }
                }
            }
        }

        if spread.is_empty() {
            break;
        }
        current = spread;
        i += 1;
    }
    i
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn neighbours(&self) -> [Point; 4] {
        let mut neighbours = [*self; 4];
        neighbours[0].x -= 1;
        neighbours[1].x += 1;
        neighbours[2].y -= 1;
        neighbours[3].y += 1;
        neighbours
    }
    fn neighbour(&self, direction: Direction) -> Point {
        let mut neighbour = *self;
        match direction {
            Direction::Up => neighbour.y += 1,
            Direction::Down => neighbour.y -= 1,
            Direction::Right => neighbour.x += 1,
            Direction::Left => neighbour.x -= 1,
        }
        neighbour
    }
    fn is_neighbour(&self, other: Point) -> bool {
        (self.x == other.x && (other.y - self.y).abs() == 1)
            || (self.y == other.y && (other.x - self.x).abs() == 1)
    }
    // gives in what direction is other
    fn direction(&self, other: Point) -> Direction {
        if !self.is_neighbour(other) {
            #[cold]
            panic!("can only compute direction of neighbours")
        };
        if self.x < other.x {
            Direction::Right
        } else if self.x > other.x {
            Direction::Left
        } else if self.y > other.y {
            Direction::Down
        } else if self.y < other.y {
            Direction::Up
        } else {
            #[cold]
            panic!("direction of self is not very nice")
        }
    }
    fn dist_to(&self, other: Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
impl Direction {
    fn to_int(&self) -> i64 {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum State {
    Wall,
    Empty,
    Tank,
}
struct Robot {
    logic: IntcodeComputer,
    position: Point,
    known: HashMap<Point, State>,
    boundry: HashSet<Point>,
}

impl Robot {
    fn new(computer: IntcodeComputer) -> Self {
        let mut known = HashMap::new();
        known.insert(Point { x: 0, y: 0 }, State::Empty);
        let mut boundry = HashSet::new();
        boundry.insert(Point { x: 0, y: 1 });
        boundry.insert(Point { x: 0, y: -1 });
        boundry.insert(Point { x: -1, y: 0 });
        boundry.insert(Point { x: 1, y: 0 });
        Self {
            logic: computer,
            position: Point { x: 0, y: 0 },
            known,
            boundry,
        }
    }
    fn reinit_computer(&mut self, computer: IntcodeComputer) {
        self.logic = computer;
        self.position = Point { x: 0, y: 0 };
    }
    fn step(&mut self, direction: Direction) -> State {
        self.logic.add_input(direction.to_int());
        loop {
            match self.logic.step() {
                IntcodeState::Ready => continue,
                IntcodeState::Outputed => match self.logic.output().last() {
                    Some(&0) => break State::Wall,
                    Some(&1) => break State::Empty,
                    Some(&2) => break State::Tank,
                    #[cold]
                    Some(i) => panic!("Invalid finding: {}", i),
                    #[cold]
                    None => panic!("You lied"),
                },
                #[cold]
                IntcodeState::Finished => panic!("robot stopped"),
                #[cold]
                IntcodeState::NeedsInput => panic!("input should have been given"),
            }
        }
    }
    fn step_and_update(&mut self, direction: Direction) -> State {
        let state = self.step(direction);
        let polled_position = self.position.neighbour(direction);
        self.boundry.remove(&polled_position);
        self.known.insert(polled_position, state);
        if state == State::Tank || state == State::Empty {
            for new_neigbour in &polled_position.neighbours() {
                if !self.known.contains_key(&new_neigbour) {
                    self.boundry.insert(*new_neigbour);
                }
            }
            self.position = polled_position;
        }
        state
    }
    fn step_on_empty(&mut self, direction: Direction) {
        let state = self.step(direction);
        let new_position = self.position.neighbour(direction);
        if let State::Wall = state {
            self.panic("You told to walk on empty");
        }
        self.position = new_position;
    }
    fn draw(&self, special_points: HashMap<Point, yansi::Paint<&str>>) {
        let min_x = self.known.keys().map(|p| p.x).min().unwrap() - 1;
        let min_y = self.known.keys().map(|p| p.y).min().unwrap() - 1;

        let max_x = self.known.keys().map(|p| p.x).max().unwrap() + 1;
        let max_y = self.known.keys().map(|p| p.y).max().unwrap() + 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = Point { x, y };
                if point == self.position {
                    print!("{}", yansi::Paint::new("☺").bold());
                } else if special_points.contains_key(&point) {
                    print!("{}", special_points.get(&point).unwrap());
                } else {
                    print!(
                        "{}",
                        match self.known.get(&point) {
                            Some(&State::Wall) => yansi::Paint::new(" ").bg(yansi::Color::Red),
                            Some(&State::Empty) => yansi::Paint::new(" "),
                            Some(&State::Tank) => yansi::Paint::new("T").bold().underline(),
                            None => yansi::Paint::new(" ").bg(yansi::Color::Cyan),
                        }
                    )
                }
            }
            print!("\n");
        }
    }
    fn explore(&mut self) -> Option<State> {
        if self.boundry.is_empty() {
            return None;
        }
        let neighbours = self.position.neighbours();
        for neighbour in &neighbours {
            if self.boundry.contains(neighbour) {
                return Some(self.step_and_update(self.position.direction(*neighbour)));
            }
        }
        // If we are here it's that there are no neighbouring unknows, we need to find a new one
        self.go_next_to_boundry();
        self.explore()
    }
    fn go_next_to_boundry(&mut self) {
        let point = self
            .boundry
            .iter()
            .min_by(|&&p, &&q| self.position.dist_to(p).cmp(&self.position.dist_to(q)))
            .unwrap();
        for neighbour in &point.neighbours() {
            match self.known.get(neighbour) {
                Some(&State::Empty) | Some(&State::Tank) => {
                    self.go_to_by_explored(*neighbour);
                    return;
                }
                _ => continue,
            }
        }
    }
    fn go_to_by_explored(&mut self, goal: Point) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.position);
        let mut paths = Vec::new();
        for &neighbour in &self.position.neighbours() {
            match self.known.get(&neighbour) {
                Some(&State::Empty) | Some(&State::Tank) => {
                    let direction = self.position.direction(neighbour);
                    if neighbour == goal {
                        self.step_on_empty(direction);
                        return 1;
                    } else {
                        paths.push(vec![(neighbour, direction)]);
                        visited.insert(neighbour);
                    }
                }
                None | Some(&State::Wall) => continue,
            }
        }
        let path = 'outer: loop {
            let mut new_paths = Vec::new();
            for path in paths {
                if path.is_empty() {
                    self.panic("empty paths in go_to_by_explored");
                }
                let (point, _) = path.last().unwrap();
                for &neighbour in &point.neighbours() {
                    if !visited.contains(&neighbour) {
                        match self.known.get(&neighbour) {
                            Some(&State::Empty) | Some(&State::Tank) => {
                                let mut path = path.clone();
                                path.push((neighbour, point.direction(neighbour)));
                                if neighbour == goal {
                                    break 'outer path;
                                } else {
                                    visited.insert(neighbour);
                                    new_paths.push(path);
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
            paths = new_paths;
        };
        let len = path.len();
        for (_, dir) in path {
            self.step_on_empty(dir);
        }
        len
    }
    fn panic(&self, msg: &str) -> ! {
        self.draw(HashMap::new());
        panic!("Incorret stop: {}", msg);
    }
}
