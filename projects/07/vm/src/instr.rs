use std::fmt;

pub enum Instr {
    Push(Segment, u16),
    Add,
}

pub enum Segment {
    Const,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Push(segment, i) => match segment {
                Segment::Const => format!(
                    "\
@{i}
D=A
@SP
A=M
M=D
@SP
M=M+1"
                ),
            },
            Self::Add => "add".into(),
        };

        write!(f, "{}", s)
    }
}
