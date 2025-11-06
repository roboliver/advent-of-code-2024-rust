use crate::PartOutput;
use std::borrow::Cow;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::{fmt, fs, io};

pub trait Day {
    fn read_input(&'_ self) -> Result<Cow<'_, str>, ReadError>;
    fn day_num(&self) -> u8;
    fn part_1_name(&self) -> &'static str;
    fn part_2_name(&self) -> &'static str;
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
    pub part_1_name: &'static str,
    pub part_2_name: &'static str,
    pub part_1: fn(&str) -> T,
    pub part_2: fn(&str) -> U,
}

impl<T: Display, U: Display> Day for DaySpec<T, U> {
    fn read_input(&'_ self) -> Result<Cow<'_, str>, ReadError> {
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

    fn part_1_name(&self) -> &'static str {
        self.part_1_name
    }

    fn part_2_name(&self) -> &'static str {
        self.part_2_name
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
    fn read_input(&'_ self) -> Result<Cow<'_, str>, ReadError> {
        Ok(Cow::Borrowed(""))
    }

    fn day_num(&self) -> u8 {
        self.day_num
    }

    fn part_1_name(&self) -> &'static str {
        "TODO"
    }

    fn part_2_name(&self) -> &'static str {
        "TODO"
    }

    fn run_part_1(&self, _input: &str) -> PartOutput {
        PartOutput::Todo
    }

    fn run_part_2(&self, _input: &str) -> PartOutput {
        PartOutput::Todo
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point { pub x: isize, pub y: isize }

impl Point {
    pub fn in_bounds(&self, width: usize, length: usize) -> bool {
        self.x >= 0 && self.x < isize::try_from(width).unwrap() &&
            self.y >= 0 && self.y < isize::try_from(length).unwrap()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction { North, East, South, West }

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn rotate_anticlockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dimensions { pub width: usize, pub length: usize }

