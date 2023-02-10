use std::fmt;

pub enum Instr {
    Push(Segment, u16),
    Add,
    Sub,
    Neg,
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
            Self::Add => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1"
                .into(),
            Self::Sub => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1"
                .into(),
            Self::Neg => "\
@SP
M=M-1
A=M
M=-M
@SP
M=M+1"
                .into(),
        };

        write!(f, "{}", s)
    }
}
