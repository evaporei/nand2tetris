use std::iter::Peekable;
use std::str::Chars;

type Symbol = String;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    A(Symbol),
    C(CInstruction),
}

impl Instruction {
    pub fn a(chars: Peekable<Chars>) -> Self {
        let symbol = chars.skip(1).collect();
        Instruction::A(symbol)
    }

    pub fn c(mut chars: Peekable<Chars>) -> Self {
        let dest = if let Some(idx) = chars.by_ref().position(|ch| ch == '=') {
            Some(chars.by_ref().take(idx).collect::<String>())
        } else {
            None
        };

        let comp = chars.by_ref().take_while(|ch| *ch != ';').collect();

        let jump = if let Some(idx) = chars.by_ref().position(|ch| ch == ';') {
            Some(chars.by_ref().take(idx).collect::<String>())
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
    let a_ins = Instruction::a(a_txt.chars().peekable());

    assert_eq!(a_ins, Instruction::A("123".to_string()));
    assert_eq!(a_ins.to_string(), "0000000001111011")
}
