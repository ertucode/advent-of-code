use std::{collections::HashSet, fs};

fn q1_nice(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut num_of_vowels = 0;
    let mut last_letter = '0';
    let forbidden = ["ab", "cd", "pq", "xy"];
    let mut has_two_consec = false;

    for cha in input.chars() {
        if vowels.contains(&cha) {
            num_of_vowels += 1
        }
        if forbidden.contains(&format!("{}{}", &last_letter, cha).as_str()) {
            return false;
        }

        if &last_letter == &cha {
            has_two_consec = true
        }
        last_letter = cha;
    }
    if has_two_consec && num_of_vowels > 2 {
        return true;
    }
    return false;
}

pub fn q2_nice(input: &str) -> bool {
    let mut chars = input.chars();

    let mut last_two: [char; 2] = [chars.next().unwrap(), chars.next().unwrap()];

    let mut pairs: HashSet<[char; 2]> = HashSet::new();

    let mut three_thing = false;
    let mut pair_thing = false;

    let mut no_check = 0;

    pairs.insert(last_two.clone());
    for cha in chars {
        let mut last_three_eq = false;
        if cha == last_two[0] {
            three_thing = true;
            if cha == last_two[1] {
                last_three_eq = true;
            };
        }
        last_two = [last_two[1], cha];

        if last_three_eq {
            no_check += 1;
            if no_check == 1 {
                continue;
            } else {
                pair_thing = true;
                no_check = 0;
                continue;
            }
        }

        if pairs.contains(&last_two) {
            pair_thing = true;
        } else {
            pairs.insert(last_two.clone());
        }
        no_check = 0;
    }

    if three_thing && pair_thing {
        return true;
    }

    false
}

pub fn q1() -> usize {
    let input = fs::read_to_string("./src/y2015/day5-input.txt").unwrap();

    input
        .lines()
        .fold(0, |acc, line| if q1_nice(&line) { acc + 1 } else { acc })
}
pub fn q2() -> usize {
    let input = fs::read_to_string("./src/y2015/day5-input.txt").unwrap();

    input
        .lines()
        .fold(0, |acc, line| if q2_nice(&line) { acc + 1 } else { acc })
}
