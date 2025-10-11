use std::borrow::Cow;
use crate::common::DaySpec;

pub const DAY_TWO: DaySpec<usize, usize> = DaySpec { day_num: 2, part_1, part_2 };

pub fn part_1(input: &str) -> usize {
    let reports = parse_input(input);
    reports.iter()
        .filter(|&report| is_safe(report))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let reports = parse_input(input);
    reports.iter()
        .filter(|&report| is_safe_actual(report))
        .count()
}

fn is_safe(report: &[u8]) -> bool {
    let report_ascending: Cow<[u8]> = if report[0] < report[1] {
        Cow::Borrowed(report)
    } else {
        Cow::Owned(report.iter().rev().copied().collect())
    };
    report_ascending.windows(2)
        .all(|pair| is_level_pair_safe_ascending(pair[0], pair[1]))
}

fn is_safe_actual(report: &[u8]) -> bool {
    if is_safe_actual_ascending(report) {
        return true;
    }
    let report_reversed: Vec<u8> = report.iter().rev().copied().collect();
    is_safe_actual_ascending(&report_reversed)
}

fn is_safe_actual_ascending(report_ascending: &[u8]) -> bool {
    let mut bad_level_found = false;
    let mut i = 0;
    while i < report_ascending.len() - 1 {
        if !is_level_pair_safe_ascending(report_ascending[i], report_ascending[i + 1]) {
            if bad_level_found {
                return false;
            }
            bad_level_found = true;
            let can_ignore = ignore_first(i) ||
                ignore_last(report_ascending, i) ||
                ignore_current(report_ascending, i) ||
                ignore_next(report_ascending, i);
            if !can_ignore {
                return false;
            }
            if ignore_next(report_ascending, i) {
                i += 1;
            }
        }
        i += 1;
    }
    true
}

fn ignore_first(i: usize) -> bool {
    i == 0
}

fn ignore_last(report_ascending: &[u8], i: usize) -> bool {
    i == report_ascending.len() - 2
}

fn ignore_current(report_ascending: &[u8], i: usize) -> bool {
    i > 0 &&
        is_level_pair_safe_ascending(report_ascending[i - 1], report_ascending[i + 1])
}

fn ignore_next(report_ascending: &[u8], i: usize) -> bool {
    i < report_ascending.len() - 2 &&
        is_level_pair_safe_ascending(report_ascending[i], report_ascending[i + 2])
}

fn is_level_pair_safe_ascending(first: u8, second: u8) -> bool {
    let diff = i16::from(second) - i16::from(first);
    diff >= 1 && diff <= 3
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
     input.lines()
        .map(|line| line.split(" ").into_iter()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_1_sample() {
        assert_eq!(2, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(4, part_2(INPUT));
    }
}