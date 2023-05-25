use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
    str::FromStr,
};

struct Relation {
    cities: Cities,
    dist: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRelationError;

impl FromStr for Relation {
    type Err = ParseRelationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" = ").unwrap();
        let (a, b) = left.split_once(" to ").unwrap();

        Ok(Relation {
            cities: Cities(a.to_owned(), b.to_owned()),
            dist: right.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Cities(String, String);
impl PartialEq for Cities {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.0 == other.1)
    }
}
impl Eq for Cities {}
impl Hash for Cities {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.0.le(&self.1) {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day9-input.txt").unwrap();

    let mut relations: HashMap<Cities, usize> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    input.lines().for_each(|l| {
        let r: Relation = l.parse().unwrap();
        if !cities.contains(&r.cities.0) {
            cities.insert(r.cities.0.clone());
        }
        cities.insert(r.cities.1.clone());
        relations.insert(r.cities, r.dist);
    });

    let perms = cities.iter().permutations(cities.len());

    perms.fold(usize::MAX, |acc, perm| {
        let mut first = perm[0];
        let mut total_dist: usize = 0;
        for &thing in perm[1..].iter() {
            let c = Cities(first.to_owned(), thing.to_owned());
            total_dist += relations.get(&c).expect(&format!("Expected: {:?}", c));
            first = thing;
        }
        std::cmp::min(total_dist, acc)
    })
}

pub fn q2() -> usize {
    let input = fs::read_to_string("./src/y2015/day9-input.txt").unwrap();

    let mut relations: HashMap<Cities, usize> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    input.lines().for_each(|l| {
        let r: Relation = l.parse().unwrap();
        if !cities.contains(&r.cities.0) {
            cities.insert(r.cities.0.clone());
        }
        cities.insert(r.cities.1.clone());
        relations.insert(r.cities, r.dist);
    });

    let perms = cities.iter().permutations(cities.len());

    perms.fold(0, |acc, perm| {
        let mut first = perm[0];
        let mut total_dist: usize = 0;
        for &thing in perm[1..].iter() {
            let c = Cities(first.to_owned(), thing.to_owned());
            total_dist += relations.get(&c).expect(&format!("Expected: {:?}", c));
            first = thing;
        }
        std::cmp::max(total_dist, acc)
    })
}
