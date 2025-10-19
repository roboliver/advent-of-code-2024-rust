use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::common::DaySpec;

pub const DAY_FIVE: DaySpec<u32, u32> = DaySpec { day_num: 5, part_1, part_2 };

pub fn part_1(input: &str) -> u32 {
    let (page_ordering_rules, updates) = parse_input(input);
    updates.iter()
        .filter(|update| is_ordered(&page_ordering_rules, update))
        .map(|update| middle_page(update))
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let (page_ordering_rules, mut updates) = parse_input(input);
    updates.iter_mut()
        .filter(|update| !is_ordered(&page_ordering_rules, update))
        .map(|update| {
            order_pages(&page_ordering_rules, update);
            middle_page(&update)
        })
        .sum()
}

fn order_pages(page_ordering_rules: &HashMap<u32, HashSet<u32>>, update: &mut [u32]) {
    update.sort_unstable_by(|&page_1, &page_2| {
        let page_1_before_2 = page_ordering_rules.get(&page_2)
            .map_or(false, |pages_before_2| pages_before_2.contains(&page_1));
        let page_2_before_1 = page_ordering_rules.get(&page_1)
            .map_or(false, |pages_before_1| pages_before_1.contains(&page_2));
        match (page_1_before_2, page_2_before_1) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    });
}

fn is_ordered(page_ordering_rules: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    let mut expected_already = HashSet::<u32>::new();
    for &page in update {
        if expected_already.contains(&page) {
            return false;
        }
        if let Some(prev_pages) = page_ordering_rules.get(&page) {
            for &prev_page in prev_pages {
                expected_already.insert(prev_page);
            }
        }
    }
    true
}

fn middle_page(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn parse_input(input: &str) -> (
    HashMap<u32, HashSet<u32>>,
    Vec<Vec<u32>>,
) {
    let mut page_ordering_rules = HashMap::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next().filter(|line| !line.is_empty()) {
        let rule = line.split("|")
            .map(|page| page.parse().unwrap())
            .collect::<Vec<u32>>();
        let (first, second) = (rule[0], rule[1]);
        page_ordering_rules.entry(second).or_insert(HashSet::new()).insert(first);
    }
    let updates = lines.map(
        |line| line.split(",")
            .map(|page| page.parse().unwrap())
            .collect())
        .collect();
    (page_ordering_rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_1_sample() {
        assert_eq!(143, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(123, part_2(INPUT));
    }
}