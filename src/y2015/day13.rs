use std::{fs, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Feeling {
    from: String,
    to: String,
    amount: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseFeelingError {}

impl FromStr for Feeling {
    type Err = ParseFeelingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let to = right.replace('.', "");
        let (from, right) = left.split_once(" would ").unwrap();
        let amount: isize;

        if right.starts_with("gain") {
            amount = right.replace("gain ", "").parse().unwrap();
        } else {
            amount = -right.replace("lose ", "").parse::<isize>().unwrap();
        }

        Ok(Feeling {
            to,
            from: from.to_string(),
            amount,
        })
    }
}

fn happiness_between(feelings: &Vec<Feeling>, left: &str, right: &str) -> isize {
    let from_left = feelings
        .iter()
        .find(|f| f.from == *left && f.to == *right)
        .unwrap()
        .amount;
    let from_right = feelings
        .iter()
        .find(|f| f.from == *right && f.to == *left)
        .unwrap()
        .amount;
    from_left + from_right
}

fn calc_overall_happiness(vec: &Vec<&String>, feelings: &Vec<Feeling>) -> isize {
    let without_first_last = vec.iter().tuple_windows().fold(0, |acc, (left, right)| {
        acc + happiness_between(&feelings, left, right)
    });

    without_first_last + happiness_between(feelings, vec.first().unwrap(), vec.last().unwrap())
}

fn do_q1(s: &str) -> isize {
    let feelings = s.lines().map(|l| l.parse::<Feeling>().unwrap()).collect();

    let mut people: Vec<String> = s
        .lines()
        .map(|l| l.split_once(" ").unwrap().0.to_string())
        .collect();

    people.dedup();

    people
        .iter()
        .permutations(people.len())
        .fold(isize::MIN, |acc, trial| {
            let happ = calc_overall_happiness(&trial, &feelings);
            std::cmp::max(acc, happ)
        })
}

pub fn q1() -> isize {
    let input = fs::read_to_string("./src/y2015/day13-input.txt").unwrap();
    do_q1(&input)
}

pub fn q2() -> isize {
    let input = fs::read_to_string("./src/y2015/day13-input.txt").unwrap();
    let mut people: Vec<String> = input
        .lines()
        .map(|l| l.split_once(" ").unwrap().0.to_string())
        .collect();

    let mut feelings: Vec<Feeling> = input
        .lines()
        .map(|l| l.parse::<Feeling>().unwrap())
        .collect();

    people.dedup();

    let me = "me".to_string();
    for person in people.iter() {
        feelings.push(Feeling {
            from: me.clone(),
            to: person.clone(),
            amount: 0,
        });
        feelings.push(Feeling {
            to: me.clone(),
            from: person.clone(),
            amount: 0,
        });
    }
    people.push(me);

    people
        .iter()
        .permutations(people.len())
        .fold(isize::MIN, |acc, trial| {
            let happ = calc_overall_happiness(&trial, &feelings);
            std::cmp::max(acc, happ)
        })
}

#[test]
fn test_q1() {
    let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    assert_eq!(do_q1(&input), 330);
}
