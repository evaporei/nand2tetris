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
            .fold(SymbolTable::with_predefined(), |mut acc, s| {
                acc.insert(s.s);
                acc
            })
    }

    pub fn parse_symbol(idx: usize, instruction: &str) -> Option<Symbol> {
        let mut instruction = instruction.trim_start().chars();
        let first = instruction.nth(0);
        let chars: String = instruction.take_while(|&ch| ch != '/').collect();
        let chars = chars.trim_end().to_owned();

        match first {
            Some('/') => None,
            Some('@') => Symbol::variable(idx, chars),
            Some('(') => Symbol::label(idx, chars),
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
        let mut instruction = instruction.trim_start().chars();
        let first = instruction.nth(0);
        let chars: String = instruction.take_while(|&ch| ch != '/').collect();
        let mut chars = chars.trim_end().to_owned();

        match first {
            Some('/') => None,
            Some('@') => Some(Instruction::a(chars, symbol_table)),
            Some('(') => None,
            Some(c) => {
                chars.insert(0, c);
                Some(Instruction::c(chars))
            }
            None => None,
        }
    }
}

#[cfg(test)]
use crate::symbol_table::SymbolKind;

#[test]
fn test_parse_symbol() {
    let symbol = Parser::parse_symbol(0, "@R0").unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Variable,
        }
    );
}

#[test]
fn test_parse_symbol_trim() {
    let symbol = Parser::parse_symbol(0, "  \n\t  @R0   \n  ").unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Variable,
        }
    );
}

#[test]
fn test_parse_symbol_comment() {
    let symbol = Parser::parse_symbol(0, "@R0 // hello  ").unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Variable,
        }
    );
    let symbol = Parser::parse_symbol(0, "  @R0//hello  ").unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Variable,
        }
    );
}
