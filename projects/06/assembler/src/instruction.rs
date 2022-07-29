use std::iter::Peekable;
use std::str::Chars;

pub enum Instruction {
    A(String),
}

impl Instruction {
    pub fn a(chars: Peekable<Chars>) -> Self {
        let symbol = chars.skip(1).collect();
        Instruction::A(symbol)
    }
}

use std::fmt;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "empty instruction")
    }
}
