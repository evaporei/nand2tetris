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
        let chars = instruction.chars();
        let mut peekable_chars = chars.clone().peekable();
        let first = peekable_chars.peek().copied();
        let second = peekable_chars.peek();

        match first {
            Some('/') if second == Some(&'/') => None,
            Some('@') => Some(Instruction::a(chars)),
            Some(_c) => Some(Instruction::c(chars)),
            None => None,
        }
    }
}
