use std::{fs, str::FromStr};

#[derive(Debug)]
struct Deer {
    fly_for: (usize, usize),
    rest: usize,
    points: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseDeerError {}

impl FromStr for Deer {
    type Err = ParseDeerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" seconds, but then must rest for ").unwrap();

        let (rest, _) = right.split_once(" seconds").unwrap();
        let rest: usize = rest.parse().unwrap();

        let (left, ffor) = left.split_once(" km/s for ").unwrap();
        let (_, fly) = left.split_once("can fly ").unwrap();

        Ok(Deer {
            fly_for: (fly.parse().unwrap(), ffor.parse().unwrap()),
            rest,
            points: 0,
        })
    }
}

impl Deer {
    fn distance_traveled(&self, total_sec: usize) -> usize {
        let fly_in_one_go = self.fly_for.0 * self.fly_for.1;
        let fly_and_rest = self.fly_for.1 + self.rest;

        let (num_fly_and_rest, rem_time) = (total_sec / fly_and_rest, total_sec % fly_and_rest);

        num_fly_and_rest * fly_in_one_go + std::cmp::min(rem_time * self.fly_for.0, fly_in_one_go)
    }
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day14-input.txt").unwrap();

    let deers = input.lines().map(|l| l.parse::<Deer>().unwrap());

    deers.into_iter().fold(usize::MIN, |acc, deer| {
        let res = std::cmp::max(acc, deer.distance_traveled(2503));
        res
    })
}
pub fn q2() -> usize {
    let input = fs::read_to_string("./src/y2015/day14-input.txt").unwrap();

    let mut deers: Vec<Deer> = input.lines().map(|l| l.parse::<Deer>().unwrap()).collect();

    for dist in 1..=2503 {
        let info_and_deer = deers.iter_mut().map(|d| (d.distance_traveled(dist), d));

        let mut max_info: (usize, Vec<&mut Deer>) = (usize::MIN, Vec::new());

        for (dist, deer) in info_and_deer {
            if dist < max_info.0 {
                continue;
            } else if dist == max_info.0 {
                max_info.1.push(deer);
            } else {
                max_info.0 = dist;
                max_info.1 = vec![deer]
            }
        }

        for ele in max_info.1.iter_mut() {
            ele.points = ele.points + 1;
        }
    }
    deers.into_iter().max_by_key(|d| d.points).unwrap().points
}

#[test]
fn test_distance() {
    let comet = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds"
        .parse::<Deer>()
        .unwrap();
    let dancer = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds"
        .parse::<Deer>()
        .unwrap();

    assert_eq!(comet.distance_traveled(1000), 1120);
    assert_eq!(dancer.distance_traveled(1000), 1056);
}
