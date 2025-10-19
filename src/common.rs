use std::borrow::Cow;
use std::fmt::Display;
use std::{fmt, fs, io};
use crate::PartOutput;

pub trait Day {
    fn read_input(&self) -> Result<Cow<str>, ReadError>;
    fn day_num(&self) -> u8;
    fn run_part_1(&self, input: &str) -> PartOutput;
    fn run_part_2(&self, input: &str) -> PartOutput;
}

#[derive(Debug)]
pub enum ReadError {
    DayError(u8),
    FileError(io::Error)
}

impl Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ReadError::DayError(day_um) => {
                write!(f, "{} is not a valid day: day values must be between 1 and 25", day_um)
            },
            ReadError::FileError(e) => {
                write!(f, "cannot read file: {}", e.to_string())
            },
        }
    }
}

#[derive(Clone)]
pub struct DaySpec<T: Display, U: Display> {
    pub day_num: u8,
    pub part_1: fn(&str) -> T,
    pub part_2: fn(&str) -> U,
}

impl<T: Display, U: Display> Day for DaySpec<T, U> {
    fn read_input(&self) -> Result<Cow<str>, ReadError> {
        let day_num = self.day_num;
        if day_num == 0 || day_num > 25 {
            return Err(ReadError::DayError(day_num));
        }
        fs::read_to_string(format!("input/day_{:02}.txt", day_num))
            .map(|s| Cow::Owned(s))
            .map_err(|e| ReadError::FileError(e))
    }

    fn day_num(&self) -> u8 {
        self.day_num
    }

    fn run_part_1(&self, input: &str) -> PartOutput {
        PartOutput::Impl(
            (self.part_1)(input).to_string()
        )
    }

    fn run_part_2(&self, input: &str) -> PartOutput {
        PartOutput::Impl(
            (self.part_2)(input).to_string()
        )
    }
}

pub struct DaySpecTodo {
    pub day_num: u8
}

impl Day for DaySpecTodo {
    fn read_input(&self) -> Result<Cow<str>, ReadError> {
        Ok(Cow::Borrowed(""))
    }

    fn day_num(&self) -> u8 {
        self.day_num
    }

    fn run_part_1(&self, _input: &str) -> PartOutput {
        PartOutput::Todo
    }

    fn run_part_2(&self, _input: &str) -> PartOutput {
        PartOutput::Todo
    }
}
