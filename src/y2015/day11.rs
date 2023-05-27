fn is_okay_q1(s: &str) -> bool {
    let mut has_increasing = false;
    let mut num_pairs = 0;

    let mut prev_3 = '_';
    let mut prev_2 = '_';
    let mut prev_1 = '_';

    for c in s.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        };

        let iprev_2 = prev_2 as u8;
        let iprev_1 = prev_1 as u8;
        let ic = c as u8;

        if (iprev_2 + 1) == iprev_1 && (iprev_1 + 1) == ic {
            has_increasing = true;
        }

        if c == prev_1 {
            if prev_2 == prev_3 || prev_2 != prev_1 {
                num_pairs += 1;
            }
        }
        prev_3 = prev_2;
        prev_2 = prev_1;
        prev_1 = c;
    }
    has_increasing && num_pairs > 1
}

fn increment_str(s: &str) -> String {
    let mut rev_chars = s.chars().rev();
    let mut incremented = String::new();

    while let Some(c) = rev_chars.next() {
        if c == 'z' {
            incremented.push('a');
        } else {
            incremented.push(increment_char(&c));
            return format!(
                "{}{}",
                rev_chars.rev().collect::<String>(),
                incremented.chars().rev().collect::<String>()
            );
        }
    }
    incremented.push('a');
    incremented.chars().rev().collect::<String>()
}

fn increment_char(c: &char) -> char {
    if *c == 'z' {
        return 'a';
    };
    (c.to_owned() as u8 + 1) as char
}

fn next_q1(s: &str) -> String {
    let mut text = s.to_string();

    loop {
        text = increment_str(&text);
        if is_okay_q1(&text) {
            return text;
        };
    }
}

pub fn q1() -> String {
    next_q1("cqjxjnds")
}
pub fn q2() -> String {
    next_q1(&next_q1("cqjxjnds"))
}

#[test]
fn test_inc_char() {
    assert_eq!(increment_char(&'a'), 'b');
    assert_eq!(increment_char(&'z'), 'a');
}

#[test]
fn test_inc_str() {
    assert_eq!(increment_str("aaaaaa"), "aaaaab");
    assert_eq!(increment_str("zzzzzz"), "aaaaaaa");
    assert_eq!(increment_str("asdasdz"), "asdasea");
}

#[test]
fn test_is_okay_q1() {
    assert!(is_okay_q1("abcdffaa"));
    assert!(is_okay_q1("ghjaabcc"));
    assert!(!is_okay_q1("ghjaabcb"));
}
