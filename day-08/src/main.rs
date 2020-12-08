use common::Puzzle;
use regex::Regex;
use std::str::FromStr;
use std::string::ParseError;
use std::mem::discriminant;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(Default)]
struct CPU {
    acc: i64,
    pc: i64,
}

#[derive(Copy, Clone)]
struct Patch {
    find: Instruction,
    replace: Instruction,
    start_at: usize,
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Copy, Clone,Eq, PartialEq, Debug)]
enum TerminationReason {
    Loop,
    Complete,
}

impl Instruction {
    fn operand(&self) -> i64 {
        match self {
            Self::Acc(operand) => *operand,
            Self::Jmp(operand) => *operand,
            Self::Nop(operand) => *operand,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        let parser = Regex::new(r"^(.+?) (\+|-)(\d+)").unwrap();
        let captures = parser.captures(s).unwrap();

        let instr = captures.get(1).unwrap().as_str();
        let sign = captures.get(2).unwrap().as_str();
        let operand: i64 = match sign {
            "-" => -captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            "+" => captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            _ => panic!(),
        };

        match instr {
            "acc" => Ok(Instruction::Acc(operand)),
            "jmp" => Ok(Instruction::Jmp(operand)),
            "nop" => Ok(Instruction::Nop(operand)),
            _ => panic!(),
        }
    }
}

#[derive(Default)]
struct Puzzle1 {
    cpu: CPU,
    program: Vec<Instruction>,

    patch: Option<Patch>
}

impl Puzzle1 {
    fn compile_and_add_line(&mut self, line: &str) {
        self.program.push(line.parse::<Instruction>().unwrap());
    }

    fn decode(&self) -> Instruction {
        let original_inst = self.program[self.cpu.pc as usize];
        match self.patch {
            Some(p) if p.start_at == self.cpu.pc as usize => {
                match p.replace {
                    Instruction::Acc(_) => Instruction::Acc(original_inst.operand()),
                    Instruction::Jmp(_) => Instruction::Jmp(original_inst.operand()),
                    Instruction::Nop(_) => Instruction::Nop(original_inst.operand()),
                }
            },
            _ => original_inst,
        }
    }

    fn run_till_loop(&mut self) -> TerminationReason {
        let mut visited_memory = Vec::new();
        while !visited_memory.contains(&self.cpu.pc) {
            let pc = self.cpu.pc as usize;
            let pc_change = match self.decode() {
                Instruction::Acc(operand) => {
                    self.cpu.acc += operand;
                    1
                },
                Instruction::Jmp(operand) => operand,
                Instruction::Nop(_) => 1,
            };
            visited_memory.push(pc as i64);
            self.cpu.pc += pc_change;

            if self.cpu.pc as usize == self.program.len() {
                return TerminationReason::Complete;
            }
        }

        TerminationReason::Loop
    }

    fn reset(&mut self) {
        self.cpu.acc = 0;
        self.cpu.pc = 0;
        self.patch = None;
    }

    fn patch(&mut self, patch: &Patch) -> Option<usize> {
        for mem_i in patch.start_at..self.program.len() {
            let mem_instr = self.program[mem_i];
            if discriminant(&mem_instr) == discriminant(&patch.find) {
                self.patch = Some(Patch { find: patch.find, replace: patch.replace, start_at: mem_i });
                return Some(mem_i);
            }
        }

        None
    }

    fn try_patch(&mut self, patch: &mut Patch) -> TerminationReason {
        self.reset();
        let mut replaced_at = self.patch(patch);
        while replaced_at.is_some() {
            patch.start_at = replaced_at.unwrap() + 1;
            match self.run_till_loop() {
                TerminationReason::Complete => return TerminationReason::Complete,
                TerminationReason::Loop => {
                    self.reset();
                    replaced_at = self.patch(&patch)
                },
            };
        }
        TerminationReason::Loop
    }

    fn try_to_correct(&mut self) -> TerminationReason {
        let mut patch_1 = Patch { find: Instruction::Jmp(0), replace: Instruction::Nop(0), start_at: 0 };
        let mut patch_2 = Patch { find: Instruction::Nop(0), replace: Instruction::Jmp(0), start_at: 0 };

        match self.try_patch(&mut patch_1) {
            TerminationReason::Loop => self.try_patch(&mut patch_2),
            TerminationReason::Complete => TerminationReason::Complete,
        }
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.compile_and_add_line(&item);
    }

    fn final_result(&mut self) -> String {
        self.run_till_loop();
        let part_a = self.cpu.acc;
        let part_b = match self.try_to_correct() {
            TerminationReason::Complete => self.cpu.acc,
            TerminationReason::Loop => panic!(),
        };

        format!("a: {}; b: {}", part_a, part_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.run_with_input(input);
        subject.reset();
        subject.run_till_loop();

        assert_eq!(9, subject.program.len());
        assert_eq!(5, subject.cpu.acc);
    }

    #[test]
    fn example_2() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.run_with_input(input);
        let result = subject.try_to_correct();

        assert_eq!(result, TerminationReason::Complete);
        assert_eq!(8, subject.cpu.acc);
    }
}
