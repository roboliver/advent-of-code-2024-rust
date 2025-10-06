use std::borrow::Cow;
use advent_of_code_2024_rust::common::PartOutput;
use advent_of_code_2024_rust::days;

fn main() {
    let days = days();
    for day in days {
        println!("Day {}", day.day_num());
        let input = day.read_input().unwrap();
        println!("Part 1: {}", part_output_str(day.run_part_1(&input)));
        println!("Part 2: {}", part_output_str(day.run_part_2(&input)));
        println!();
    }
}

fn part_output_str(output: PartOutput) -> Cow<'static, str> {
    match output {
        PartOutput::Impl(str) => Cow::Owned(str),
        PartOutput::Todo => Cow::Borrowed("TODO"),
    }
}