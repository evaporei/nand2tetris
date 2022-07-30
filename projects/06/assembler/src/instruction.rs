use std::str::Chars;

type Symbol = String;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    A(Symbol),
    C(CInstruction),
}

impl Instruction {
    pub fn a(chars: Chars) -> Self {
        let symbol = chars.skip(1).collect();
        Instruction::A(symbol)
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
    #[allow(dead_code)]
    dest: Option<Symbol>,
    #[allow(dead_code)]
    comp: Symbol,
    #[allow(dead_code)]
    jump: Option<Symbol>,
}

use std::fmt;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(address) => write!(f, "0{:015b}", address.parse::<u16>().unwrap()),
            Self::C(_c_instr) => write!(f, "111"),
        }
    }
}

#[test]
fn test_a() {
    let a_txt = "@123";
    let a_ins = Instruction::a(a_txt.chars());

    assert_eq!(a_ins, Instruction::A("123".to_string()));
    assert_eq!(a_ins.to_string(), "0000000001111011");
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
    assert_eq!(c_ins.to_string(), "111");
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
    assert_eq!(c_ins.to_string(), "111");
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
    assert_eq!(c_ins.to_string(), "111");
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
    assert_eq!(c_ins.to_string(), "111");
}
