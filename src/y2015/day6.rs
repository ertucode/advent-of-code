use regex::Regex;
use std::{fs, str::FromStr};

enum InstructionType {
    On,
    Off,
    Toggle,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionTypeError;
impl FromStr for InstructionType {
    type Err = ParseInstructionTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(InstructionType::On),
            "turn off" => Ok(InstructionType::Off),
            "toggle" => Ok(InstructionType::Toggle),
            _ => Err(ParseInstructionTypeError),
        }
    }
}

struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').expect("aoc lies");
        Ok(Point(x.parse().unwrap(), y.parse().unwrap()))
    }
}

struct Instruction {
    start: Point,
    end: Point,
    inst_type: InstructionType,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructError;

impl FromStr for Instruction {
    type Err = ParseInstructError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?P<inst_type>.*) (?P<start>\d{1,3},\d{1,3}) through (?P<end>\d{1,3},\d{1,3})",
        )
        .unwrap();
        let captures = re.captures(&s).unwrap();

        let start = captures.name("start").unwrap().as_str();
        let end = captures.name("end").unwrap().as_str();
        let inst_type = captures.name("inst_type").unwrap().as_str().trim();

        Ok(Instruction {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
            inst_type: inst_type.parse().unwrap(),
        })
    }
}

impl Instruction {
    fn apply_bool(&self, arr: &mut [[bool; 1000]]) {
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                arr[x][y] = match &self.inst_type {
                    InstructionType::On => true,
                    InstructionType::Off => false,
                    InstructionType::Toggle => !arr[x][y],
                }
            }
        }
    }

    fn apply_num(&self, arr: &mut [[usize; 1000]]) {
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                let val = arr[x][y];
                arr[x][y] = match &self.inst_type {
                    InstructionType::On => val + 1,
                    InstructionType::Off => {
                        if val == 0 {
                            0
                        } else {
                            val - 1
                        }
                    }
                    InstructionType::Toggle => val + 2,
                }
            }
        }
    }
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day6-input.txt").unwrap();

    let mut arr = [[false; 1000]; 1000];

    input.lines().for_each(|line| {
        let instruction: Instruction = line.parse().unwrap();
        instruction.apply_bool(&mut arr);
    });

    let mut total: usize = 0;
    for x in 0..arr.len() {
        let inner = arr[x];
        for y in 0..inner.len() {
            if inner[y] {
                total += 1;
            }
        }
    }
    total
}
pub fn q2() -> usize {
    let input = fs::read_to_string("./src/y2015/day6-input.txt").unwrap();

    let mut arr: [[usize; 1000]; 1000] = [[0; 1000]; 1000];

    input.lines().for_each(|line| {
        let instruction: Instruction = line.parse().unwrap();
        instruction.apply_num(&mut arr);
    });

    let mut total: usize = 0;
    for x in 0..arr.len() {
        let inner = arr[x];
        for y in 0..inner.len() {
            total += inner[y];
        }
    }
    total
}
