use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs;

pub fn q1() -> i32 {
    let input = fs::read_to_string("./src/y2015/day1-input.txt").unwrap();
    let res = input
        .chars()
        .fold(1, |acc, x| if x == '(' { acc + 1 } else { acc - 1 });
    return res;
}

pub fn q2() -> i32 {
    let input = fs::read_to_string("./src/y2015/day1-input.txt").unwrap();
    let res = input
        .chars()
        .enumerate()
        .fold_while(1 as i32, |acc, (idx, x)| {
            if x == '(' {
                Continue(acc + 1)
            } else {
                let c = acc - 1;
                if c == -1 {
                    Done(idx.try_into().unwrap())
                } else {
                    Continue(c)
                }
            }
        })
        .into_inner();
    return res;
}
