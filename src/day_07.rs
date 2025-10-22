use crate::common::DaySpec;
use num::pow;

pub const DAY_SEVEN: DaySpec<u64, u64> = DaySpec {
    day_num: 7,
    part_1_name: "total calibration result",
    part_1,
    part_2_name: "total calibration result (including concat)",
    part_2,
};

fn part_1(input: &str) -> u64 {
    parse_input(input).iter_mut()
        .map(|equation| {
            if has_solution(equation.test, &mut equation.numbers, false) {
                Some(equation.test)
            } else {
                None
            }
        })
        .filter(|test| test.is_some())
        .map(Option::unwrap)
        .sum()
}

fn part_2(input: &str) -> u64 {
    parse_input(input).iter_mut()
        .map(|equation| {
            if has_solution(equation.test, &mut equation.numbers, true) {
                Some(equation.test)
            } else {
                None
            }
        })
        .filter(|test| test.is_some())
        .map(Option::unwrap)
        .sum()
}

fn has_solution(test: u64, numbers: &mut Vec<u64>, with_concat: bool) -> bool {
    let last = numbers.pop().unwrap();
    let solution_found = if numbers.is_empty() {
        test == last
    } else {
        (with_concat && last_matches(test, last) && has_solution(remove_last(test, last), numbers, true)) ||
            (test % last == 0 && has_solution(test / last, numbers, with_concat)) ||
            has_solution(test - last, numbers, with_concat)
    };
    numbers.push(last);
    solution_found
}

fn last_matches(test: u64, last: u64) -> bool {
    test.to_string().ends_with(&last.to_string())
}

fn remove_last(test: u64, last: u64) -> u64 {
    test / pow(10, last.to_string().len())
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(parse_equation)
        .collect()
}

fn parse_equation(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(": ").take(2).collect();
    let (test, numbers) = (parts[0], parts[1]);
    Equation {
        test: test.parse().unwrap(),
        numbers: numbers.split(" ")
            .map(|num| num.parse().unwrap())
            .collect()
    }
}

struct Equation {
    test: u64,
    numbers: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part_1_sample() {
        assert_eq!(3749, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(11387, part_2(INPUT));
    }
}
