use crate::instruction::{InstrKind, Instruction};
use crate::symbol_table::{Symbol, SymbolTable};
use std::str::Lines;

pub struct Parser {}

impl Parser {
    pub fn first_pass(program_lines: Lines) -> (Vec<(InstrKind, String)>, SymbolTable) {
        let (_, lines, symbol_table) = program_lines.fold(
            (0, vec![], SymbolTable::with_predefined()),
            |(mut idx, mut lines, mut acc), instr| {
                let (instr_kind, instr, symbol) = Self::parse_symbol(idx, instr);
                if let Some(instr_kind) = instr_kind {
                    idx += 1;
                    lines.push((instr_kind, instr))
                }
                if let Some(s) = symbol {
                    acc.insert(s);
                }
                (idx, lines, acc)
            },
        );
        (lines, symbol_table)
    }

    // Just looks for the label indexes
    pub fn parse_symbol(
        idx: usize,
        instruction: &str,
    ) -> (Option<InstrKind>, String, Option<Symbol>) {
        let mut instruction = instruction.trim_start().chars();
        let first = instruction.nth(0);
        let chars: String = instruction.take_while(|&ch| ch != '/').collect();
        let mut chars = chars.trim_end().to_owned();

        match first {
            Some('/') => (None, chars, None),
            Some('@') => (Some(InstrKind::A), chars, None), // variables are handled in second pass
            Some('(') => (None, chars.clone(), Symbol::label(idx, chars)), // it's not an instruction
            Some(c) => {
                chars.insert(0, c);
                (Some(InstrKind::C), chars, None)
            }
            None => (None, chars, None),
        }
    }

    pub fn second_pass(
        program_lines: Vec<(InstrKind, String)>,
        mut symbol_table: SymbolTable,
    ) -> Vec<String> {
        program_lines
            .into_iter()
            .enumerate()
            .map(|(idx, (kind, line))| Self::parse_line(kind, line, idx, &mut symbol_table))
            .filter_map(|x| x)
            .map(|i| i.to_string())
            .collect()
    }

    fn parse_line(
        kind: InstrKind,
        instruction: String,
        idx: usize,
        symbol_table: &mut SymbolTable,
    ) -> Option<Instruction> {
        match kind {
            InstrKind::A => {
                if let Some(var) = Symbol::variable(idx, instruction.clone()) {
                    symbol_table.insert(var);
                }
                Some(Instruction::a(instruction, symbol_table))
            }
            InstrKind::C => Some(Instruction::c(instruction)),
        }
    }
}

#[cfg(test)]
use crate::symbol_table::SymbolKind;

#[test]
fn test_parse_symbol() {
    let symbol = Parser::parse_symbol(0, "(R0)").2.unwrap();
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
    let symbol = Parser::parse_symbol(0, "  \n\t  (R0)  \n  ").2.unwrap();
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
    let symbol = Parser::parse_symbol(0, "(R0) // hello  ").2.unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Label,
        }
    );
    let symbol = Parser::parse_symbol(0, "  (R0)//hello  ").2.unwrap();
    assert_eq!(
        symbol,
        Symbol {
            idx: 0,
            s: "R0".into(),
            k: SymbolKind::Label,
        }
    );
}
