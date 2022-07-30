use crate::instruction::Instruction;
use crate::symbol_table::{Symbol, SymbolTable};
use std::str::Lines;

pub struct Parser {}

impl Parser {
    pub fn first_pass(program_lines: Lines) -> SymbolTable {
        program_lines
            .enumerate()
            .map(|(idx, inst)| Self::parse_symbol(idx, inst))
            .filter_map(|x| x)
            .map(Symbol::to_tuple)
            .collect()
    }

    fn parse_symbol(idx: usize, instruction: &str) -> Option<Symbol> {
        let chars = instruction.chars();
        let mut peekable_chars = chars.clone().peekable();
        let first = peekable_chars.peek().copied();
        let second = peekable_chars.peek();

        match first {
            Some('/') if second == Some(&'/') => None,
            Some('@') => Symbol::new(idx, chars),
            Some(_c) => None,
            None => None,
        }
    }

    pub fn second_pass(program_lines: Lines, symbol_table: SymbolTable) -> Vec<String> {
        program_lines
            .map(|line| Self::parse_line(line, &symbol_table))
            .filter_map(|x| x)
            .map(|i| i.to_string())
            .collect()
    }

    fn parse_line(instruction: &str, symbol_table: &SymbolTable) -> Option<Instruction> {
        let chars = instruction.chars();
        let mut peekable_chars = chars.clone().peekable();
        let first = peekable_chars.peek().copied();
        let second = peekable_chars.peek();

        match first {
            Some('/') if second == Some(&'/') => None,
            Some('@') => Some(Instruction::a(chars, symbol_table)),
            Some(_c) => Some(Instruction::c(chars)),
            None => None,
        }
    }
}
