use crate::common::{Day, DaySpecTodo};
use crate::day_01::DAY_ONE;
use crate::day_02::DAY_TWO;
use crate::day_03::DAY_THREE;

mod day_01;
mod day_02;
mod day_03;
mod common;

pub fn days<'a>() -> Vec<Box<dyn Day>> {
    let mut days: Vec<Box<dyn Day>> = vec![
        Box::new(DAY_ONE.clone()),
        Box::new(DAY_TWO.clone()),
        Box::new(DAY_THREE.clone()),
    ];
    for day_todo_num in days.len()..=25 {
        let day_todo = DaySpecTodo {
            day_num: u8::try_from(day_todo_num).unwrap()
        };
        days.push(Box::new(day_todo));
    };
    days
}

pub enum PartOutput {
    Impl(String),
    Todo,
}
