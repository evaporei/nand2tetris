use crate::instruction::Instruction;
use std::str::Lines;

pub struct Parser {}

impl Parser {
    pub fn parse(program_lines: Lines) -> Vec<String> {
        program_lines
            .map(Self::line)
            .filter_map(|x| x)
            .map(|i| i.to_string())
            .collect()
    }

    fn line(instruction: &str) -> Option<Instruction> {
        let mut chars = instruction.chars().peekable();

        match chars.peek() {
            Some('@') => Some(Instruction::a(chars)),
            Some(_c) => Some(Instruction::c(chars)),
            None => None,
        }
    }
}
