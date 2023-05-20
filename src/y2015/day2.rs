use std::{fs, str::FromStr};

struct Box {
    l: i32,
    w: i32,
    h: i32,
}

impl FromStr for Box {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split('x').collect();

        let l = splits[0].parse().unwrap();
        let w = splits[1].parse().unwrap();
        let h = splits[2].parse().unwrap();

        return Ok(Box { l, w, h });
    }
}

impl Box {
    fn paper(&self) -> i32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l + self.smallest_area()
    }

    fn smallest_area(&self) -> i32 {
        let smallest = self.smallest_two();
        smallest.0 * smallest.1
    }

    fn ribbon(&self) -> i32 {
        self.w * self.h * self.l + self.smallest_perim()
    }

    fn smallest_perim(&self) -> i32 {
        let smallest = self.smallest_two();
        smallest.0 + smallest.0 + smallest.1 + smallest.1
    }

    fn smallest_two(&self) -> (i32, i32) {
        let mut sorted = [self.w, self.h, self.l];
        sorted.sort();

        (sorted[0], sorted[1])
    }
}

pub fn q1() -> i32 {
    let input = fs::read_to_string("./src/y2015/day2-input.txt").unwrap();

    input
        .lines()
        .map(str::parse::<Box>)
        .map(Result::unwrap)
        .into_iter()
        .fold(0, |acc, x| acc + x.paper())
}

pub fn q2() -> i32 {
    let input = fs::read_to_string("./src/y2015/day2-input.txt").unwrap();

    input
        .lines()
        .map(str::parse::<Box>)
        .map(Result::unwrap)
        .into_iter()
        .fold(0, |acc, x| acc + x.ribbon())
}
