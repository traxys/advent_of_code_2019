use num_rational::Rational;
use std::collections::{HashSet, HashMap};

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn dist_squared(&self, other: &Point) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

#[aoc_generator(day10)]
pub fn get_asteroids(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| Point {
                    x: j as isize,
                    y: i as isize,
                })
        })
        .fold(HashSet::new(), |mut set, points| {
            set.extend(points);
            set
        })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    a: Rational,
    b: Rational,
    c: Rational,
}
impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.b == Rational::from_integer(0) || other.b == Rational::from_integer(0) {
            if other.b == Rational::from_integer(0) {
                Some(std::cmp::Ordering::Equal)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        } else {
            Some(self.a.cmp(&other.a))
        }
    }
}

fn line_through(pa: Point, pb: Point) -> Line {
    if pa.x == pb.x {
        Line {
            a: Rational::from_integer(1),
            b: Rational::from_integer(0),
            c: Rational::from_integer(pa.x),
        }
    } else {
        let a = Rational::new(pb.y - pa.y, pb.x - pa.x);
        let b = Rational::from_integer(-1);
        Line {
            a,
            b,
            c: a * pa.x + b * pa.y,
        }
    }
}
fn visible_in_line(source: Point, points: &HashSet<Point>) -> usize {
    let mut more = false;
    let mut less = false;
    for p in points {
        if p.x == source.x {
            if p.y > source.y {
                more = true;
            }
            if p.y < source.y {
                less = true;
            }
        } else {
            if p.x > source.x {
                more = true;
            }
            if p.x < source.x {
                less = true;
            }
        }
        if more && less {
            break;
        }
    }
    more as usize + less as usize
}
fn find_asteroid_in_view_from(
    point: Point,
    asteroids: &HashSet<Point>,
) -> (usize, HashMap<Line, HashSet<Point>>) {
    let mut lines = HashMap::new();
    for asteroid in asteroids {
        let line = line_through(point, *asteroid);
        lines
            .entry(line)
            .or_insert_with(HashSet::new)
            .insert(*asteroid);
    }
    (
        lines
            .values()
            .map(|line| visible_in_line(point, line))
            .sum(),
        lines,
    )
}
fn find_most_beacony(asteroids: &HashSet<Point>) -> (usize, HashMap<Line, HashSet<Point>>, Point) {
    asteroids
        .iter()
        .map(|source| {
            let (len, lines) = find_asteroid_in_view_from(*source, asteroids);
            (len, lines, *source)
        })
        .max_by(|(len1, ..), (len2, ..)| len1.cmp(len2))
        .unwrap()
}
#[aoc(day10, part1)]
pub fn find_beacon_asteroid(input: &HashSet<Point>) -> usize {
    let (asteroids_in_sight, _, location) = find_most_beacony(input);
    println!("Station is at: ({:#?})", location);
    asteroids_in_sight
}

fn lesser_upper(source: Point, line: &HashSet<Point>) -> (Vec<Point>, Vec<Point>) {
    let mut more = Vec::new();
    let mut less = Vec::new();
    for p in line {
        if p.x == source.x {
            if p.y < source.y {
                more.push(*p);
            }
            if p.y > source.y {
                less.push(*p);
            }
        } else {
            if p.x > source.x {
                more.push(*p);
            }
            if p.x < source.x {
                less.push(*p);
            }
        }
    }
    let dist_cmp = |point: &Point, other: &Point| {
        other
            .dist_squared(&source)
            .cmp(&point.dist_squared(&source))
    };
    more.sort_unstable_by(dist_cmp);
    less.sort_unstable_by(dist_cmp);
    (more, less)
}

#[aoc(day10, part2)]
pub fn find_lasered(input: &HashSet<Point>) -> isize {
    let station = Point {
        x: 26,
        y: 36,
    };
    let (_, lines) = find_asteroid_in_view_from(station, input);
    //let (_, lines, station) = find_most_beacony(input);
    let mut lines: Vec<_> = lines
        .into_iter()
        .map(|(l, line)| {
            let (more, less) = lesser_upper(station, &line);
            (l, more, less)
        })
        .collect();
    lines.sort_unstable_by(|(line, _, _), (other_line, _, _)| {
        line.partial_cmp(&other_line).unwrap()
    });
    let mut lines: Vec<_> = lines
        .into_iter()
        .map(|(_, more, less)| (more, less))
        .collect();
    let mut right = true;
    let mut i = 0;
    let point = 'outer: loop {
        for (more, less) in &mut lines {
            let side = if right { more } else { less };
            if let Some(point) = side.pop() {
                i += 1;
                if i == 200 {
                    break 'outer point;
                }
            }
        }
        right = !right;
    };
    100 * point.x + point.y
}
