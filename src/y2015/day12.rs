use itertools::Itertools;
use serde_json::Value;
use std::fs;

use regex::Regex;

fn has_red(obj: &serde_json::Map<std::string::String, Value>) -> bool {
    obj.values().contains(&Value::String(String::from("red")))
}

fn do_q2(val: &Value) -> isize {
    match val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::String(str) => {
            if let Ok(num) = str.parse() {
                num
            } else {
                0
            }
        }
        Value::Array(items) => items.to_vec().into_iter().fold(0, |acc, v| acc + do_q2(&v)),
        Value::Number(num) => num.as_i64().unwrap() as isize,
        Value::Object(obj) => {
            if has_red(&obj) {
                0 as isize
            } else {
                obj.values()
                    .into_iter()
                    .fold(0 as isize, |acc, v| acc + do_q2(v))
            }
        }
    }
}

pub fn q1() -> isize {
    let input = fs::read_to_string("./src/y2015/day12-input.txt").unwrap();

    let regex = Regex::new(r"-?\d+").unwrap();

    regex.captures_iter(&input).fold(0, |acc, s| {
        let str = s.get(0).unwrap().as_str();
        acc + str.parse::<isize>().unwrap()
    })
}
pub fn q2() -> isize {
    let input = fs::read_to_string("./src/y2015/day12-input.txt").unwrap();

    let obj: Value = serde_json::from_str(&input).unwrap();

    do_q2(&obj)
}
