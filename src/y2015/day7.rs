use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Put(WireOrNumber, String),
    And((WireOrNumber, WireOrNumber), String),
    Or((WireOrNumber, WireOrNumber), String),
    LShift((String, u16), String),
    RShift((String, u16), String),
    Not(WireOrNumber, String),
}

#[derive(Debug)]
enum WireOrNumber {
    Wire(String),
    Number(u16),
}

impl FromStr for WireOrNumber {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse::<u16>() {
            return Ok(WireOrNumber::Number(number));
        } else {
            return Ok(WireOrNumber::Wire(s.to_owned()));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();
        let right = right.to_owned();

        if left.starts_with("NOT") {
            let (_, wire) = left.split_once(" ").unwrap();
            return Ok(Instruction::Not(wire.parse().unwrap(), right.to_owned()));
        }

        if !left.contains(' ') {
            return Ok(Instruction::Put(left.parse().unwrap(), right.to_owned()));
        }

        let (first, comm_sec) = left.split_once(" ").unwrap();
        let (comm, sec) = comm_sec.split_once(" ").unwrap();

        let first = first.to_owned();
        let sec = sec.to_owned();

        match comm {
            "AND" => Ok(Instruction::And(
                (first.parse().unwrap(), sec.parse().unwrap()),
                right,
            )),
            "OR" => Ok(Instruction::Or(
                (first.parse().unwrap(), sec.parse().unwrap()),
                right,
            )),
            "LSHIFT" => Ok(Instruction::LShift((first, sec.parse().unwrap()), right)),
            "RSHIFT" => Ok(Instruction::RShift((first, sec.parse().unwrap()), right)),
            _ => Err(ParseInstructionError),
        }
    }
}

struct Circuit {
    state: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            state: HashMap::new(),
        }
    }

    fn apply(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Or((x, y), d) => self
                .state
                .insert(d.to_owned(), self.get_value(&x) | self.get_value(&y)),
            Instruction::And((x, y), d) => self
                .state
                .insert(d.to_owned(), self.get_value(&x) & self.get_value(&y)),
            Instruction::LShift((x, y), d) => self
                .state
                .insert(d.to_owned(), self.state.get(x).unwrap_or(&0) << y),
            Instruction::RShift((x, y), d) => self
                .state
                .insert(d.to_owned(), self.state.get(x).unwrap_or(&0) >> y),
            Instruction::Put(num, x) => self.state.insert(x.to_owned(), self.get_value(num)),
            Instruction::Not(x, y) => self.state.insert(y.to_owned(), !self.get_value(&x)),
        };
    }
    fn get_value(&self, wn: &WireOrNumber) -> u16 {
        match &wn {
            WireOrNumber::Number(num) => num.to_owned(),
            WireOrNumber::Wire(wire) => self.state.get(wire).unwrap_or(&0).to_owned(),
        }
    }
}

pub fn q1() -> u16 {
    let input = fs::read_to_string("./src/y2015/day7-input.txt").unwrap();
    let mut circuit = Circuit::new();

    input.lines().for_each(|line| {
        let inst: Instruction = line.parse().unwrap();
        circuit.apply(&inst);
    });

    circuit.state.get("a").unwrap_or(&0).to_owned()
}
pub fn q2() -> usize {
    2
}
