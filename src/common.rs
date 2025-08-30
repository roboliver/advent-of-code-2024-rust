use std::fmt::Display;
use std::fs;

pub struct DaySpec<T: Display, U: Display> {
    pub day_num: u8,
    pub part_1: fn(&str) -> T,
    pub part_2: fn(&str) -> U,
}

pub trait Day {
    fn read_input(&self) -> Result<String, String>;
    fn day_num(&self) -> u8;
    fn run_part_1(&self, input: &str) -> String;
    fn run_part_2(&self, input: &str) -> String;
    fn matches_part_1(&self, input: &str, expected: &str) -> bool;
    fn matches_part_2(&self, input: &str, expected: &str) -> bool;
}

impl<T: Display, U: Display> Day for DaySpec<T, U> {
    fn read_input(&self) -> Result<String, String> {
        let day = self.day_num;
        if day == 0 || day > 25 {
            return Err(String::from("day must be between 1 and 25"));
        }
        fs::read_to_string(format!("input/day_{:02}.txt", day))
            .map_err(|e| String::from(format!("could not open file: {}", e.to_string())))
    }

    fn day_num(&self) -> u8 {
        self.day_num
    }

    fn run_part_1(&self, input: &str) -> String {
        format!("{}", (self.part_1)(input))
    }

    fn run_part_2(&self, input: &str) -> String {
        format!("{}", (self.part_2)(input))
    }

    fn matches_part_1(&self, input: &str, expected: &str) -> bool {
        self.run_part_1(input).eq(expected)
    }

    fn matches_part_2(&self, input: &str, expected: &str) -> bool {
        self.run_part_2(input).eq(expected)
    }
}
