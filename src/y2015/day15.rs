use std::{fs, str::FromStr};

use regex::Regex;

struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseIngredientError {}

impl FromStr for Ingredient {
    type Err = ParseIngredientError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex =
            Regex::new(r": capacity (-?\d*), durability (-?\d*), flavor (-?\d*), texture (-?\d*), calories (-?\d*)")
                .unwrap();

        let cap = regex.captures_iter(s).next().unwrap();

        Ok(Ingredient {
            capacity: cap[0].parse().unwrap(),
            durability: cap[1].parse().unwrap(),
            flavor: cap[2].parse().unwrap(),
            texture: cap[3].parse().unwrap(),
            calories: cap[4].parse().unwrap(),
        })
    }
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day15-input.txt").unwrap();

    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(|l| l.parse::<Ingredient>().unwrap())
        .collect();

    2
}
pub fn q2() -> usize {
    2
}
