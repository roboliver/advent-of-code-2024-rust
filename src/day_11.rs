use std::collections::HashMap;
use crate::common::DaySpec;

pub const DAY_ELEVEN: DaySpec<u64, u64> = DaySpec {
    day_num: 11,
    part_1_name: "stones after 25 blinks",
    part_1,
    part_2_name: "stones after 75 blinks",
    part_2
};

fn part_1(input: &str) -> u64 {
    let stone_counts = parse_input(input);
    calculate_stones(stone_counts, 25)
}

fn part_2(input: &str) -> u64 {
    let stone_counts = parse_input(input);
    calculate_stones(stone_counts, 75)
}

fn calculate_stones(mut stone_counts: HashMap<u64, u64>, blinks: u8) -> u64 {
    for _ in 0..blinks {
        let mut new_stone_counts = HashMap::new();
        for (&stone, &count) in stone_counts.iter() {
            if stone == 0 {
                update_stone(&mut new_stone_counts, 1, count);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let left_new_stone_str = &stone_str[0..stone_str.len() / 2];
                let right_new_stone_str = &stone_str[(stone_str.len() / 2)..];
                update_stone(&mut new_stone_counts, left_new_stone_str.parse().unwrap(), count);
                update_stone(&mut new_stone_counts, right_new_stone_str.parse().unwrap(), count);
            } else {
                update_stone(&mut new_stone_counts, stone * 2024, count);
            }
        }
        stone_counts = new_stone_counts;
        let mut my_vec = Vec::new();
        for val in &stone_counts {
            my_vec.push(val.1.clone());
        }
        my_vec.sort();
    }
    stone_counts.values().sum()
}

fn update_stone(stone_counts: &mut HashMap<u64, u64>, stone: u64, count: u64) {
    stone_counts.entry(stone)
        .and_modify(|old| *old += count)
        .or_insert(count);
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    input.split(" ")
        .map(|stone| (stone.parse().unwrap(), 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(55312, part_1("125 17"));
    }
}