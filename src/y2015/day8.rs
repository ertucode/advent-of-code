use std::fs;

fn code_minus_mem(s: &str) -> usize {
    let mut diff: usize = 2;
    let mut chars = s.chars();

    loop {
        if let Some(c) = chars.next() {
            if c == '\\' {
                let next = chars.next().unwrap();
                if next == '\"' || next == '\\' {
                    diff += 1;
                } else if next == 'x' {
                    chars.next().unwrap();
                    chars.next().unwrap();
                    diff += 3;
                }
            }
        } else {
            break diff;
        }
    }
}

fn encoded_minus_code(s: &str) -> usize {
    let mut diff: usize = 4;
    let len = s.len();
    let mut chars = s[1..len - 1].chars();

    loop {
        if let Some(c) = chars.next() {
            if c == '\"' || c == '\\' {
                diff += 1;
            }
        /*
        else if c == '\\' {
            let next = chars.next().unwrap();
            if next == 'x' || next == '\\' {
                diff += 2;
            }
        }
        */
        } else {
            break diff;
        }
    }
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day8-input.txt").unwrap();

    input.lines().fold(0, |acc, l| acc + code_minus_mem(&l))
}
pub fn q2() -> usize {
    let input = fs::read_to_string("./src/y2015/day8-input.txt").unwrap();

    input.lines().fold(0, |acc, l| acc + encoded_minus_code(&l))
}
