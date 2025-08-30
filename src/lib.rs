use crate::common::Day;
use crate::day_01::DAY_ONE;

pub mod day_01;
pub mod common;

pub fn days() -> Vec<&'static dyn Day> {
    vec![&DAY_ONE]
}
