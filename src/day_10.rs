use std::collections::HashSet;
use crate::common::{DaySpec, Point};

pub const DAY_TEN: DaySpec<u32, u32> = DaySpec {
    day_num: 10,
    part_1_name: "trailhead scores sum",
    part_1,
    part_2_name: "trailhead ratings sum",
    part_2,
};

fn part_1(input: &str) -> u32 {
    let topography = parse_input(input);
    let mut sum = 0;
    for row in 0..topography.len() {
        for col in 0..topography[0].len() {
            if topography[row][col] == 0 {
                sum += calculate_trailhead_score_sum(&topography, row, col) as u32;
            }
        }
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let topography = parse_input(input);
    let mut sum = 0;
    for row in 0..topography.len() {
        for col in 0..topography[0].len() {
            if topography[row][col] == 0 {
                sum += calculate_trailhead_rating_sum(&topography, row, col) as u32;
            }
        }
    }
    sum
}

fn calculate_trailhead_score_sum(topography: &[Vec<u8>], row: usize, col: usize) -> usize {
    let mut peaks = HashSet::new();
    calculate_trailheads(topography, row, col, &mut peaks);
    peaks.len()
}

fn calculate_trailhead_rating_sum(topography: &[Vec<u8>], row: usize, col: usize) -> usize {
    calculate_trailheads(topography, row, col, &mut HashSet::new())
}

fn calculate_trailheads(
    topography: &[Vec<u8>],
    row: usize,
    col: usize,
    peaks: &mut HashSet<Point>
) -> usize {
    let height = topography[row][col];
    if height == 9 {
        peaks.insert(Point {
            x: isize::try_from(col).unwrap(),
            y: isize::try_from(row).unwrap(),
        });
        return 1;
    }
    let mut rating_sum = 0;
    if row > 0 && topography[row - 1][col] == height + 1 {
        rating_sum += calculate_trailheads(topography, row - 1, col, peaks);
    }
    if row < topography.len() - 1 && topography[row + 1][col] == height + 1 {
        rating_sum += calculate_trailheads(topography, row + 1, col, peaks);
    }
    if col > 0 && topography[row][col - 1] == height + 1 {
        rating_sum += calculate_trailheads(topography, row, col - 1, peaks);
    }
    if col < topography[0].len() - 1 && topography[row][col + 1] == height + 1 {
        rating_sum += calculate_trailheads(topography, row, col + 1, peaks);
    }
    rating_sum
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part_1_sample() {
        assert_eq!(36, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(81, part_2(INPUT));
    }
}