use std::collections::{HashSet, VecDeque};
use crate::common::DaySpec;

pub const DAY_NINETEEN: DaySpec<u32, u64> = DaySpec {
    day_num: 19,
    part_1_name: "possible designs",
    part_1,
    part_2_name: "possible ways to make all designs",
    part_2,
};

fn part_1(input: &str) -> u32 {
    let (towels, designs) = parse_input(input);
    let longest_towel = longest_towel(&towels);
    designs.iter()
        .filter(|&&design| ways_to_make_design(design, longest_towel, &towels) > 0)
        .count() as u32
}

fn part_2(input: &str) -> u64 {
    let (towels, designs) = parse_input(input);
    let longest_towel = longest_towel(&towels);
    designs.iter()
        .map(|&design| ways_to_make_design(design, longest_towel, &towels))
        .sum()
}

fn longest_towel(towels: &HashSet<&str>) -> usize {
    towels.iter()
        .map(|&towel| towel.len())
        .max()
        .unwrap()
}

fn ways_to_make_design(design: &str, longest_towel: usize, towels: &HashSet<&str>) -> u64 {
    let mut arrangements = VecDeque::from(
        [ArrangementCount { arrangement: "", count: 1 }]
    );
    for i in 1..=design.len() {
        let new_arrangement = &design[..i];
        let new_arrangement_count = arrangements.iter()
            .filter(|arrangement| {
                let towel_needed = &design[arrangement.arrangement.len()..new_arrangement.len()];
                towels.contains(towel_needed)
            })
            .map(|arrangement| arrangement.count)
            .sum();
        if arrangements.len() == longest_towel {
            arrangements.pop_front();
        }
        arrangements.push_back(
            ArrangementCount { arrangement: new_arrangement, count: new_arrangement_count });
    }
    arrangements.back().unwrap().count
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let towel_line = lines.next().unwrap();
    let towels = towel_line.split(", ").collect();
    lines.next();
    let designs = lines.collect();
    (towels, designs)
}

struct ArrangementCount<'a> {
    arrangement: &'a str,
    count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part_1_sample() {
        assert_eq!(6, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(16, part_2(INPUT));
    }
}