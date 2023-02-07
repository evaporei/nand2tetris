use crate::instruction::Instruction;
use crate::symbol_table::{Symbol, SymbolTable};
use std::str::Lines;

pub struct Parser {}

impl Parser {
    pub fn first_pass(program_lines: Lines) -> (Vec<&str>, SymbolTable) {
        // dbg!(" ---- FIRST PASS ---- ");
        let (_, lines, symbol_table) = program_lines.fold(
            (0, vec![], SymbolTable::with_predefined()),
            |(mut idx, mut lines, mut acc), instr| {
                let (was_instr, symbol) = Self::parse_symbol(idx, instr);
                if was_instr {
                    idx += 1;
                    lines.push(instr)
                }
                if let Some(s) = symbol {
                    // dbg!("it's a label!", &s.s);
                    acc.insert(s);
                }
                (idx, lines, acc)
            },
        );
        // dbg!(&lines);
        // dbg!(&symbol_table);
        (lines, symbol_table)
    }

    // Just looks for the label indexes
    pub fn parse_symbol(idx: usize, instruction: &str) -> (bool, Option<Symbol>) {
        let mut instruction = instruction.trim_start().chars();
        let first = instruction.nth(0);
        let chars: String = instruction.take_while(|&ch| ch != '/').collect();
        let chars = chars.trim_end().to_owned();

        match first {
            Some('/') => (false, None),
            Some('@') => (true, None), // variables are handled in second pass
            Some('(') => (false, Symbol::label(idx, chars)), // it's not an instruction
            Some(_c) => (true, None),  // c-instruction
            None => (false, None),
        }
    }

    pub fn second_pass(program_lines: Vec<&str>, mut symbol_table: SymbolTable) -> Vec<String> {
        // dbg!(" ---- SECOND PASS ---- ");
        program_lines
            .into_iter()
            .enumerate()
            .map(|(idx, line)| Self::parse_line(line, idx, &mut symbol_table))
            .filter_map(|x| x)
            .map(|i| i.to_string())
            .collect()
    }

    fn parse_line(
        instruction: &str,
        idx: usize,
        symbol_table: &mut SymbolTable,
    ) -> Option<Instruction> {
        // dbg!(idx);
        // dbg!(instruction);
        let mut instruction = instruction.trim_start().chars();
        let first = instruction.nth(0);
        let chars: String = instruction.take_while(|&ch| ch != '/').collect();
        let mut chars = chars.trim_end().to_owned();

        match first {
            Some('@') => {
                if let Some(var) = Symbol::variable(idx, chars.clone()) {
                    // dbg!("hhh");
                    symbol_table.insert(var);
                }
                Some(Instruction::a(chars, symbol_table))
            }
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
    let symbol = Parser::parse_symbol(0, "(R0)").1.unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Label,
        }
    );
}

#[test]
fn test_parse_symbol_trim() {
    let symbol = Parser::parse_symbol(0, "  \n\t  (R0)  \n  ").1.unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Label,
        }
    );
}

#[test]
fn test_parse_symbol_comment() {
    let symbol = Parser::parse_symbol(0, "(R0) // hello  ").1.unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Label,
        }
    );
    let symbol = Parser::parse_symbol(0, "  (R0)//hello  ").1.unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Label,
        }
    );
}
