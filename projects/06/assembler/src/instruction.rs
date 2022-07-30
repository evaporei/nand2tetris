use crate::symbol_table::SymbolTable;
use std::str::Chars;

type Address = u16;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    A(Address),
    C(CInstruction),
}

impl Instruction {
    pub fn a(chars: Chars, symbol_table: &SymbolTable) -> Self {
        let symbol: String = chars.skip(1).collect();
        let maybe_address = symbol.parse::<u16>();

        if maybe_address.is_ok() {
            Instruction::A(maybe_address.unwrap())
        } else {
            let address = symbol_table
                .get(&symbol)
                .expect(&format!("didn't find variable {}", symbol));

            Instruction::A(address)
        }
    }

    pub fn c(mut chars: Chars) -> Self {
        let dest = if let Some(idx) = chars.clone().position(|ch| ch == '=') {
            let d = Some(chars.by_ref().take(idx).collect::<String>());
            let _ = chars.next(); // skip '='
            d
        } else {
            None
        };

        let comp = chars.clone().take_while(|ch| *ch != ';').collect();

        let jump = if let Some(idx) = chars.clone().position(|ch| ch == ';') {
            Some(chars.skip(idx + 1).collect::<String>())
        } else {
            None
        };

        Instruction::C(CInstruction { dest, comp, jump })
    }
}

#[derive(Debug, PartialEq)]
pub struct CInstruction {
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
}

impl CInstruction {
    fn comp(c: &str) -> u16 {
        let s = match &c[..] {
            // a = 0
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            // a = 1
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => panic!("non existent comp conversion"),
        };

        u16::from_str_radix(s, 2).unwrap()
    }

    fn dest(d: &str) -> u16 {
        let s = match &d[..] {
            "null" => "000",
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => panic!("non existent dest conversion"),
        };

        u16::from_str_radix(s, 2).unwrap()
    }

    fn jump(j: &str) -> u16 {
        let s = match &j[..] {
            "null" => "000",
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => panic!("non existent jump conversion"),
        };

        u16::from_str_radix(s, 2).unwrap()
    }
}

use std::fmt;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(address) => write!(f, "0{:015b}", address),
            Self::C(CInstruction { dest, comp, jump }) => {
                write!(f, "111")?;
                write!(f, "{:07b}", CInstruction::comp(&comp))?;
                write!(
                    f,
                    "{:03b}",
                    CInstruction::dest(dest.as_ref().unwrap_or(&"null".to_string()))
                )?;
                write!(
                    f,
                    "{:03b}",
                    CInstruction::jump(jump.as_ref().unwrap_or(&"null".to_string()))
                )?;
                Ok(())
            }
        }
    }
}

#[test]
fn test_a() {
    let empty_symbol_table = Default::default();

    let a_txt = "@123";
    let a_ins = Instruction::a(a_txt.chars(), &empty_symbol_table);

    assert_eq!(a_ins, Instruction::A(123));
    assert_eq!(a_ins.to_string(), "0000000001111011");
}

#[test]
fn test_a_in_symbol_table() {
    let mut symbol_table = SymbolTable::with_predefined();
    symbol_table.insert("VARIABLE".into());

    let a_txt = "@VARIABLE";
    let a_ins = Instruction::a(a_txt.chars(), &symbol_table);

    assert_eq!(a_ins, Instruction::A(16));
    assert_eq!(a_ins.to_string(), "0000000000010000");
}

#[test]
fn test_c_only_comp() {
    let c_txt = "D+A";
    let c_ins = Instruction::c(c_txt.chars());

    assert_eq!(
        c_ins,
        Instruction::C(CInstruction {
            dest: None,
            comp: "D+A".to_string(),
            jump: None,
        })
    );
    assert_eq!(c_ins.to_string(), "1110000010000000");
}

#[test]
fn test_c_dest() {
    let c_txt = "M=D+A";
    let c_ins = Instruction::c(c_txt.chars());

    assert_eq!(
        c_ins,
        Instruction::C(CInstruction {
            dest: Some("M".to_string()),
            comp: "D+A".to_string(),
            jump: None,
        })
    );
    assert_eq!(c_ins.to_string(), "1110000010001000");
}

#[test]
fn test_c_jump() {
    let c_txt = "D+A;JLE";
    let c_ins = Instruction::c(c_txt.chars());

    assert_eq!(
        c_ins,
        Instruction::C(CInstruction {
            dest: None,
            comp: "D+A".to_string(),
            jump: Some("JLE".to_string()),
        })
    );
    assert_eq!(c_ins.to_string(), "1110000010000110");
}

#[test]
fn test_c_dest_jump() {
    let c_txt = "M=D+A;JLE";
    let c_ins = Instruction::c(c_txt.chars());

    assert_eq!(
        c_ins,
        Instruction::C(CInstruction {
            dest: Some("M".to_string()),
            comp: "D+A".to_string(),
            jump: Some("JLE".to_string()),
        })
    );
    assert_eq!(c_ins.to_string(), "1110000010001110");
}
