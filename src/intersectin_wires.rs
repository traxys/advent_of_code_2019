
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
