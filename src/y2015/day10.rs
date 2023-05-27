pub fn look_say(input: &str, num: usize) -> usize {
    let mut curr_text = input.to_string();

    for _ in 0..num {
        let mut next_text = "".to_string();
        let mut chars = curr_text.chars();
        let mut last_char = chars.next().unwrap();
        let mut count = 1;
        for char in chars {
            if last_char == char {
                count += 1;
            } else {
                next_text.push_str(&format!("{count}{last_char}"));
                last_char = char;
                count = 1;
            }
        }

        next_text.push_str(&format!("{count}{last_char}"));
        curr_text = next_text.clone();
    }

    curr_text.len()
}

pub fn q1() -> usize {
    look_say("3113322113", 40)
}
pub fn q2() -> usize {
    look_say("3113322113", 50)
}
