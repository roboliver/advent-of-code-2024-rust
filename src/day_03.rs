use crate::common::DaySpec;

use regex::{Captures, Regex};

pub const DAY_THREE: DaySpec<u32, u32> = DaySpec {
    day_num: 3,
    part_1_name: "multiplication sum",
    part_1,
    part_2_name: "enabled multiplication sum",
    part_2,
};
const PATTERN: &str = "mul\\(([1-9][0-9]*),([1-9][0-9]*)\\)";

pub fn part_1(input: &str) -> u32 {
    let memory = parse_input(input);
    let regex = Regex::new(PATTERN).unwrap();
    regex.captures_iter(&memory)
        .map(|cap| extract_mul(&cap, 1) * extract_mul(&cap, 2))
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let memory = parse_input(input);
    let pattern = String::from("") + "(do\\(\\))|(don't\\(\\))|" + PATTERN;
    let regex = Regex::new(&pattern).unwrap();

    let mut enabled = true;
    let mut sum = 0;

    let mut iter = regex.captures_iter(&memory);
    while let Some(cap) = iter.next() {
        if let Some(_) = cap.get(1) {
            enabled = true;
        } else if let Some(_) = cap.get(2) {
            enabled = false;
        } else if enabled {
            sum += extract_mul(&cap, 3) * extract_mul(&cap, 4)
        }
    };
    sum
}

fn extract_mul(cap: &Captures, i: usize) -> u32 {
    cap.get(i).unwrap()
        .as_str()
        .parse::<u32>().unwrap()
}

fn parse_input(input: &str) -> String {
    input.lines()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part_1(input));
    }

    #[test]
    fn part_2_sample() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part_2(input));
    }
}
