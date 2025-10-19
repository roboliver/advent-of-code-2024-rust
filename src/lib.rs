use crate::common::{Day, DaySpecTodo};
use crate::day_01::DAY_ONE;
use crate::day_02::DAY_TWO;
use crate::day_03::DAY_THREE;
use crate::day_04::DAY_FOUR;
use crate::day_05::DAY_FIVE;
use crate::day_06::DAY_SIX;

mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

pub fn days<'a>() -> Vec<Box<dyn Day>> {
    let mut days: Vec<Box<dyn Day>> = vec![
        Box::new(DAY_ONE.clone()),
        Box::new(DAY_TWO.clone()),
        Box::new(DAY_THREE.clone()),
        Box::new(DAY_FOUR.clone()),
        Box::new(DAY_FIVE.clone()),
        Box::new(DAY_SIX.clone()),
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
