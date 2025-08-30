use advent_of_code_2024_rust::days;

fn main() {
    let days = days();
    for day in days {
        println!("Day {}", day.day_num());
        let input = day.read_input().unwrap();
        println!("Part 1: {}", day.run_part_1(&input));
        println!("Part 2: {}", day.run_part_2(&input));
    }
}
