use crate::common::{Day, DaySpecTodo};
use crate::day_01::DAY_ONE;
use crate::day_02::DAY_TWO;

pub mod day_01;
pub mod day_02;
pub mod common;

pub fn days<'a>() -> Vec<Box<dyn Day>> {
    let mut days: Vec<Box<dyn Day>> = vec![
        Box::new(DAY_ONE.clone()),
        Box::new(DAY_TWO.clone()),
    ];
    for day_todo_num in days.len()..=25 {
        let day_todo = DaySpecTodo {
            day_num: u8::try_from(day_todo_num).unwrap()
        };
        days.push(Box::new(day_todo));
    };
    days
}
