mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;

use crate::common::{Day, DaySpecTodo};
use crate::day_01::DAY_ONE;
use crate::day_02::DAY_TWO;
use crate::day_03::DAY_THREE;
use crate::day_04::DAY_FOUR;
use crate::day_05::DAY_FIVE;
use crate::day_06::DAY_SIX;
use crate::day_07::DAY_SEVEN;
use crate::day_08::DAY_EIGHT;
use crate::day_09::DAY_NINE;
use crate::day_10::DAY_TEN;
use crate::day_11::DAY_ELEVEN;
use crate::day_12::DAY_TWELVE;
use crate::day_13::DAY_THIRTEEN;
use crate::day_14::DAY_FOURTEEN;
use crate::day_15::DAY_FIFTEEN;
use crate::day_16::DAY_SIXTEEN;
use crate::day_17::DAY_SEVENTEEN;
use crate::day_18::DAY_EIGHTEEN;
use crate::day_19::DAY_NINETEEN;

pub fn days<'a>() -> Vec<Box<dyn Day>> {
    let mut days: Vec<Box<dyn Day>> = vec![
        Box::new(DAY_ONE.clone()),
        Box::new(DAY_TWO.clone()),
        Box::new(DAY_THREE.clone()),
        Box::new(DAY_FOUR.clone()),
        Box::new(DAY_FIVE.clone()),
        Box::new(DAY_SIX.clone()),
        Box::new(DAY_SEVEN.clone()),
        Box::new(DAY_EIGHT.clone()),
        Box::new(DAY_NINE.clone()),
        Box::new(DAY_TEN.clone()),
        Box::new(DAY_ELEVEN.clone()),
        Box::new(DAY_TWELVE.clone()),
        Box::new(DAY_THIRTEEN.clone()),
        Box::new(DAY_FOURTEEN.clone()),
        Box::new(DAY_FIFTEEN.clone()),
        Box::new(DAY_SIXTEEN.clone()),
        Box::new(DAY_SEVENTEEN.clone()),
        Box::new(DAY_EIGHTEEN.clone()),
        Box::new(DAY_NINETEEN.clone()),
    ];
    for day_todo_num in days.len()+1..=25 {
        let day_todo = DaySpecTodo {
            day_num: u8::try_from(day_todo_num).unwrap()
        };
        days.push(Box::new(day_todo));
    }
    days
}

pub enum PartOutput {
    Impl(String),
    Todo,
}
