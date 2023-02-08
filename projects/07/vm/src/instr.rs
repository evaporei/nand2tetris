use std::fmt;

pub enum Instr {
    Push(Segment, u16),
    Add,
}

pub enum Segment {
    Const,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Const => "const",
        };

        write!(f, "{}", s)
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Push(segment, i) => format!("push {segment} {i}"),
            Self::Add => "add".into(),
        };

        write!(f, "{}", s)
    }
}
