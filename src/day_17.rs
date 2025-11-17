mod computer;

use crate::common::DaySpec;
use crate::day_17::computer::Computer;

pub const DAY_SEVENTEEN: DaySpec<String, u64> = DaySpec {
    day_num: 17,
    part_1_name: "program output",
    part_1,
    part_2_name: "lowest A value that outputs the input program",
    part_2,
};

fn part_1(input: &str) -> String {
    let (reg_a, reg_b, reg_c, program) = parse_input(input);
    let mut computer = Computer::new(reg_a, reg_b, reg_c, &program);
    computer.run_program();
    computer.get_output_str()
}

fn part_2(input: &str) -> u64 {
    // Note: this function assumes that programs:
    // - have their last instruction as 3,0 (set IP to 0 unless A is 0)
    // - remove the last 3 bits from register A in each iteration
    // - make use of up to the last 10 bits of A in deciding the value to add to the output
    // - add exactly one value to the output each iteration
    // These assumptions are true for the test input, and for the main puzzle input I had, but
    // may not be for other possible valid Day 17 inputs.
    let (_, _, _, program) = parse_input(input);
    let mut reg_a_values = init_reg_a_values(&program);
    // maximum iterations: number of 3-bits that can be prepended to the initial 10-bit reg_a
    // values to fit in a u64
    for i in 0..(53 / 3) {
        let mut new_reg_a_values = Vec::new();
        for a in reg_a_values {
            for j in 0x0..0x8 {
                let a_new = j << (10 + (3 * i)) | a; // prepend j to a
                let output = run_computer_with_reg_a(a_new, &program);
                if program == output {
                    // possible reg_a values are tried in order from 0 to u64::MAX,
                    // so the first match will be the lowest
                    return a_new;
                }
                if output.len() > (i + 1) && output[i + 1] == program[i + 1] {
                    // prepended 3-bits caused a new match, so retain this for next iteration
                    new_reg_a_values.push(a_new);
                }
            }
        }
        reg_a_values = new_reg_a_values;
    }
    panic!("no valid input found");
}

fn init_reg_a_values(program: &[u8]) -> Vec<u64> {
    let mut reg_a_values = Vec::new();
    // initialise with all valid 10-bit numbers
    for a in 0x000..0x400 {
        let output = run_computer_with_reg_a(a, program);
        if output[0] == program[0] {
            reg_a_values.push(a);
        }
    }
    reg_a_values
}

fn run_computer_with_reg_a(a: u64, program: &[u8]) -> Vec<u8> {
    let mut computer = Computer::new(a, 0, 0, program);
    computer.run_program();
    computer.output
}

fn parse_input(input: &str) -> (u64, u64, u64, Vec<u8>) {
    let mut lines = input.lines();
    let reg_a = parse_register(lines.next().unwrap());
    let reg_b = parse_register(lines.next().unwrap());
    let reg_c = parse_register(lines.next().unwrap());
    lines.next();
    let program = parse_program(lines.next().unwrap());
    (reg_a, reg_b, reg_c, program)
}

fn parse_register(line: &str) -> u64 {
    line.split_once(": ").unwrap().1.parse().unwrap()
}

fn parse_program(line: &str) -> Vec<u8> {
    let program = line.split_once(": ").unwrap().1;
    program.split(",")
        .map(|it| it.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part_1(input));
    }

    #[test]
    fn part_2_sample() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(117440, part_2(input));
    }
}