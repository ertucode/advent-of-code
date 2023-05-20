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
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day3-input.txt").unwrap();
    let mut santa = Santa {
        pos: Point(0, 0),
        visited: HashSet::new(),
    };
    santa.visited.insert(Point(0, 0));

    input.chars().for_each(|x| {
        santa.visit(&x);
    });

    santa.total_visits()
}

pub fn q2() -> usize {
    2
}
