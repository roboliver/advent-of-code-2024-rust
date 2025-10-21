use std::collections::{HashMap, HashSet};
use crate::common::{DaySpec, Dimensions, Point};

pub const DAY_EIGHT: DaySpec<usize, usize> = DaySpec { day_num: 8, part_1, part_2 };

pub fn part_1(input: &str) -> usize {
    let (antennas, dimensions) = parse_input(input);
    calculate_antinodes(antennas, dimensions, true, |_, _, _| false)
}

pub fn part_2(input: &str) -> usize {
    let (antennas, dimensions) = parse_input(input);
    calculate_antinodes(antennas, dimensions, false, Point::in_bounds)
}

fn calculate_antinodes(
    antennas: HashMap<char, Vec<Point>>,
    dimensions: Dimensions,
    skip_self_antinode: bool,
    continue_func: fn(&Point, usize, usize) -> bool
) -> usize {
    let mut antinodes = HashSet::new();
    for (_, antennas_at_freq) in antennas.iter() {
        for i in 0..antennas_at_freq.len() {
            for j in 0..antennas_at_freq.len() {
                if i == j { continue; }
                let antenna_i = *antennas_at_freq.get(i).unwrap();
                let antenna_j = *antennas_at_freq.get(j).unwrap();
                let antinodes_i_j = calculate_for_antenna(
                    antenna_i,
                    antenna_j,
                    skip_self_antinode,
                    continue_func,
                    dimensions,
                );
                let antinodes_j_i = calculate_for_antenna(
                    antenna_j,
                    antenna_i,
                    skip_self_antinode,
                    continue_func,
                    dimensions,
                );
                for antinode in antinodes_i_j.into_iter().chain(antinodes_j_i.into_iter()) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn calculate_for_antenna(
    first_antenna: Point,
    second_antenna: Point,
    skip_self_antinode: bool,
    continue_func: fn(&Point, usize, usize) -> bool,
    dimensions: Dimensions,
) -> Vec<Point> {
    let mut antinodes = Vec::new();
    let period = first_antenna - second_antenna;
    let mut current_antinode = first_antenna;
    if skip_self_antinode {
        current_antinode = current_antinode + period;
    }
    let Dimensions { width, length } = dimensions;
    let mut has_run_once = false;
    while !has_run_once || continue_func(&current_antinode, width, length) {
        if current_antinode.in_bounds(width, length) {
            antinodes.push(current_antinode);
        }
        current_antinode = current_antinode + period;
        has_run_once = true;
    }
    antinodes
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, Dimensions) {
    let mut antennas = HashMap::new();
    let mut width = 0;
    let mut length = 0;
    for (row, line) in input.lines().enumerate() {
        if row == 0 {
            width = line.len();
        }
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                let antenna: Point = Point {
                    x: isize::try_from(col).unwrap(),
                    y: isize::try_from(row).unwrap(),
                };
                antennas.entry(char).or_insert(Vec::new()).push(antenna);
            }
        }
        length += 1;
    }
    (antennas, Dimensions { width, length })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part_1_sample() {
        assert_eq!(14, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(34, part_2(INPUT));
    }
}