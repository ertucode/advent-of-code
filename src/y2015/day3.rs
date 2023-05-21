use std::{collections::HashSet, fmt, fs};

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn add(&self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Clone)]
struct InvalidVisitInput;

impl fmt::Display for InvalidVisitInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid visit input")
    }
}

struct Santa {
    pos: Point,
    visited: HashSet<Point>,
}

impl Santa {
    pub fn new() -> Santa {
        let start = Point(0, 0);
        Santa {
            pos: start,
            visited: HashSet::from([start]),
        }
    }

    fn visit(&mut self, cha: &char) {
        let pos = match cha {
            '>' => Ok(self.pos.add(Point(0, 1))),
            '<' => Ok(self.pos.add(Point(0, -1))),
            '^' => Ok(self.pos.add(Point(1, 0))),
            'v' => Ok(self.pos.add(Point(-1, 0))),
            _ => Err(InvalidVisitInput),
        };

        if let Ok(pos) = pos {
            self.visited.insert(pos);
            self.pos = pos;
        }
    }

    fn total_visits(&self) -> usize {
        self.visited.len()
    }

    fn total_visits_with_partner(&self, partner: &Santa) -> usize {
        self.visited.union(&partner.visited).count()
    }
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day3-input.txt").unwrap();
    let mut santa = Santa::new();

    input.chars().for_each(|x| {
        santa.visit(&x);
    });

    santa.total_visits()
}

pub fn q2() -> usize {
    let input = fs::read_to_string("./src/y2015/day3-input.txt").unwrap();
    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();
    let mut last_is_santa = false;

    input.chars().for_each(|x| {
        if last_is_santa {
            robo_santa.visit(&x);
            last_is_santa = false;
        } else {
            santa.visit(&x);
            last_is_santa = true;
        }
    });

    santa.total_visits_with_partner(&robo_santa)
}
