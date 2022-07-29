use std::iter::Peekable;
use std::str::Chars;

pub enum Instruction {
    A(String),
    C,
}

impl Instruction {
    pub fn a(chars: Peekable<Chars>) -> Self {
        let symbol = chars.skip(1).collect();
        Instruction::A(symbol)
    }

    pub fn c(_chars: Peekable<Chars>) -> Self {
        Instruction::C
    }
}

use std::fmt;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(number) => write!(f, "{:b}", number.parse::<u16>().unwrap()),
            Self::C => write!(f, "empty"),
        }
    }
}
