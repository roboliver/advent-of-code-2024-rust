use std::collections::HashMap;
use crate::common::DaySpec;

pub const DAY_ONE: DaySpec<u32, usize> = DaySpec {
    day_num: 1,
    part_1_name: "total distance",
    part_1,
    part_2_name: "similarity score",
    part_2,
};

pub fn part_1(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    (0..left.len())
        .map(|i| left[i].abs_diff(right[i]))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (left, right) = parse_input(input);
    let right_counts: HashMap<u32, usize> = right.iter()
        .fold(HashMap::new(), |mut counts, &x| {
            *counts.entry(x).or_insert(0) += 1;
            counts
        });
    left.iter()
        .map(|&x| (x as usize) * right_counts.get(&x).unwrap_or(&0))
        .sum()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    input.lines()
        .map(|line| line.split("   "))
        .for_each(|mut pair| {
            left.push(
                pair.next().expect("missing left value")
                    .parse().expect("left value not a number")
            );
            right.push(
                pair.next().expect("missing right value")
                    .parse().expect("right value not a number")
            );
        });
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1_sample() {
        assert_eq!(11, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(31, part_2(INPUT));
    }
}