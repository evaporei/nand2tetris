use std::iter::Peekable;
use std::str::Chars;

pub enum Instruction {
    A,
}

impl Instruction {
    pub fn a(_chars: Peekable<Chars>) -> Self {
        Instruction::A
    }
}

use std::fmt;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "empty instruction")
    }
}
