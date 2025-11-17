pub struct Computer {
    ip: usize,
    registers: Registers,
    program: Vec<Instruction>,
    pub(crate) output: Vec<u8>,
}

impl Computer {
    pub fn new(reg_a: u64, reg_b: u64, reg_c: u64, program: &[u8]) -> Self {
        let registers = Registers { a: reg_a, b: reg_b, c: reg_c };
        let program = program.chunks_exact(2)
            .map(|chunk| Instruction::new(chunk[0], chunk[1] as u64))
            .collect();
        Computer { ip: 0, registers, program, output: Vec::new() }
    }

    pub fn run_program(&mut self) {
        while self.ip < self.program.len() {
            let instruction = &self.program[self.ip];
            let ip_before = self.ip;
            Computer::execute_instruction(&mut self.registers, &mut self.ip, &mut self.output, instruction);
            if ip_before == self.ip {
                self.ip += 1;
            }
        }
    }

    fn execute_instruction(
        regs: &mut Registers,
        ip: &mut usize,
        out: &mut Vec<u8>,
        ins: &Instruction
    ) {
        match ins {
            Instruction::Adv(opd) => regs.a = regs.a >> Self::combo(regs, *opd),
            Instruction::Bxl(opd) => regs.b = regs.b ^ opd,
            Instruction::Bst(opd) => regs.b = Self::combo(regs, *opd) % 8,
            Instruction::Jnz(opd) => if regs.a != 0 { *ip = (opd / 2) as usize },
            Instruction::Bxc => regs.b = regs.b ^ regs.c,
            Instruction::Out(opd) => out.push((Self::combo(regs, *opd) % 8) as u8),
            Instruction::Bdv(opd) => regs.b = regs.a >> Self::combo(regs, *opd),
            Instruction::Cdv(opd) => regs.c = regs.a >> Self::combo(regs, *opd),
        }
    }

    fn combo(registers: &Registers, operand: u64) -> u64 {
        match operand {
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => operand,
        }
    }

    pub fn get_output_str(&self) -> String {
        self.output.iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

enum Instruction {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc,
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}

impl Instruction {
    fn new(opcode: u8, operand: u64) -> Self {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!("unknown opcode {}", opcode),
        }
    }
}

