use advent_of_code_2024_rust::common::read_input;
use advent_of_code_2024_rust::day_01::{part_1, part_2};

fn main() {
    let input = read_input(1).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
