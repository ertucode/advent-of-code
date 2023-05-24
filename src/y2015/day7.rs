use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug)]
struct Binding {
    command: Command,
    dest: String,
}

#[derive(Debug, Clone)]
enum Command {
    Put(WireOrNumber),
    And(WireOrNumber, WireOrNumber),
    Or(WireOrNumber, WireOrNumber),
    LShift(String, u16),
    RShift(String, u16),
    Not(WireOrNumber),
}

#[derive(Debug, Clone)]
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

impl FromStr for Binding {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();
        let right = right.to_owned();

        if left.starts_with("NOT") {
            let (_, wire) = left.split_once(" ").unwrap();
            return Ok(Binding {
                command: Command::Not(wire.parse().unwrap()),
                dest: right.to_owned(),
            });
        }

        if !left.contains(' ') {
            return Ok(Binding {
                command: Command::Put(left.parse().unwrap()),
                dest: right.to_owned(),
            });
        }

        let (first, comm_sec) = left.split_once(" ").unwrap();
        let (comm, sec) = comm_sec.split_once(" ").unwrap();

        let first = first.to_owned();
        let sec = sec.to_owned();

        match comm {
            "AND" => Ok(Binding {
                command: Command::And(first.parse().unwrap(), sec.parse().unwrap()),
                dest: right,
            }),
            "OR" => Ok(Binding {
                command: Command::Or(first.parse().unwrap(), sec.parse().unwrap()),
                dest: right,
            }),
            "LSHIFT" => Ok(Binding {
                command: Command::LShift(first, sec.parse().unwrap()),
                dest: right,
            }),
            "RSHIFT" => Ok(Binding {
                command: Command::RShift(first, sec.parse().unwrap()),
                dest: right,
            }),
            _ => Err(ParseInstructionError),
        }
    }
}

struct Circuit {
    state: HashMap<String, Binding>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            state: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    fn add_binding(&mut self, bind: Binding) {
        let key = bind.dest.to_owned();
        self.state.insert(key, bind);
    }

    fn calculate(&mut self, wire: &str) -> u16 {
        if let Some(cached) = self.cache.get(wire) {
            return cached.to_owned();
        }
        let command = self
            .state
            .get(wire)
            .expect("Expected wire to be in circuit")
            .command
            .to_owned();
        let res = match command {
            Command::Put(thing) => self.calc_wire_or_number(&thing),
            Command::Or(left, right) => {
                self.calc_wire_or_number(&left) | self.calc_wire_or_number(&right)
            }
            Command::And(left, right) => {
                self.calc_wire_or_number(&left) & self.calc_wire_or_number(&right)
            }
            Command::LShift(left, right) => self.calculate(&left) << right,
            Command::RShift(left, right) => self.calculate(&left) >> right,
            Command::Not(thing) => !self.calc_wire_or_number(&thing),
        };

        self.cache.insert(wire.to_owned(), res);

        return res;
    }

    fn calc_wire_or_number(&mut self, thing: &WireOrNumber) -> u16 {
        match thing {
            WireOrNumber::Wire(wire) => self.calculate(&wire),
            WireOrNumber::Number(number) => number.to_owned(),
        }
    }

    fn reset(&mut self) {
        self.cache.drain();
    }
}

pub fn q1() -> u16 {
    let input = fs::read_to_string("./src/y2015/day7-input.txt").unwrap();
    let mut circuit = Circuit::new();

    input.lines().for_each(|line| {
        let inst: Binding = line.parse().unwrap();
        circuit.add_binding(inst);
    });

    circuit.calculate("a").to_owned()
}
pub fn q2() -> u16 {
    let input = fs::read_to_string("./src/y2015/day7-input.txt").unwrap();
    let mut circuit = Circuit::new();

    input.lines().for_each(|line| {
        let inst: Binding = line.parse().unwrap();
        circuit.add_binding(inst);
    });

    let a_signal = circuit.calculate("a").to_owned();

    circuit.add_binding(Binding {
        command: Command::Put(WireOrNumber::Number(a_signal)),
        dest: "b".to_owned(),
    });

    circuit.reset();

    circuit.calculate("a").to_owned()
}
