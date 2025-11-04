use crate::common::{DaySpec, Point};

pub const DAY_THIRTEEN: DaySpec<isize, isize> = DaySpec {
    day_num: 13,
    part_1_name: "fewest tokens needed",
    part_1,
    part_2_name: "actual fewest tokens needed",
    part_2,
};

fn part_1(input: &str) -> isize {
    parse_input(input).into_iter()
        .map(|claw_machine| token_spend(claw_machine, false))
        .sum()
}

fn part_2(input: &str) -> isize {
    parse_input(input).into_iter()
        .map(|claw_machine| token_spend(claw_machine, true))
        .sum()
}

fn token_spend(claw_machine: ClawMachine, unit_correction: bool) -> isize {
    let ClawMachine {
        button_a: Point { x: a_x, y: a_y },
        button_b: Point { x: b_x, y: b_y },
        prize: Point { x: mut prize_x, y: mut prize_y }
    } = claw_machine;
    if unit_correction {
        prize_x += 10_000_000_000_000;
        prize_y += 10_000_000_000_000;
    }
    let a_numerator = (b_x * prize_y) - (b_y * prize_x);
    let a_denominator = (a_y * b_x) - (a_x * b_y);
    let b_numerator = (a_x * prize_y) - (a_y * prize_x);
    let b_denominator = (a_x * b_y) - (a_y * b_x);
    if a_numerator % a_denominator != 0 || b_numerator % b_denominator != 0 {
        return 0;
    }
    let a_presses = a_numerator / a_denominator;
    let b_presses = b_numerator / b_denominator;
    if a_presses < 1 || b_presses < 1 ||
        (!unit_correction && (a_presses > 100 || b_presses > 100)) {
        return 0;
    }
    (a_presses * 3) + b_presses
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();
    let mut lines = input.lines();
    loop {
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        let claw_machine = ClawMachine::parse(button_a, button_b, prize);
        claw_machines.push(claw_machine);
        if lines.next().is_none() {
            break;
        }
    }
    claw_machines
}

struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl ClawMachine {
    fn parse(button_a_str: &str, button_b_str: &str, prize_str: &str) -> Self {
        let button_a = ClawMachine::parse_line(button_a_str);
        let button_b = ClawMachine::parse_line(button_b_str);
        let prize = ClawMachine::parse_line(prize_str);
        Self { button_a, button_b, prize }
    }

    fn parse_line(line: &str) -> Point {
        let parts: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1]
            .split(", ").collect();
        let x = parts[0][2..].parse().unwrap();
        let y = parts[1][2..].parse().unwrap();
        Point { x, y }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(480, part_1(input));
    }
}