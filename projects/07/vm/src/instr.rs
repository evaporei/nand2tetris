use std::fmt;

pub enum Instr {
    Push(Segment, u16),
    Add,
    Sub,
    Neg,
    Eq(usize),
    Gt(usize),
    Lt(usize),
    And,
    Or,
    Not,
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
            // arithmetic
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
            // logical
            Self::Eq(i) => format!(
                "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@LOGICAL_EQ_BODY_{i}
D;JEQ

// (IF-else) -> no equality found
@SP
A=M
M=-1
@LOGICAL_EQ_END_{i}
0;JMP

// (IF-then) it's equal to zero, which means equality
(LOGICAL_EQ_BODY_{i})
@SP
A=M
M=0

(LOGICAL_EQ_END_{i})
@SP
M=M+1"
            ),
            Self::Gt(i) => format!(
                "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@LOGICAL_GT_BODY_{i}
D;JGE

// (IF-else) -> no greater than found
@SP
A=M
M=-1
@LOGICAL_GT_END_{i}
0;JMP

// (IF-then) it's greater than or equal to zero
(LOGICAL_GT_BODY_{i})
@SP
A=M
M=0

(LOGICAL_GT_END_{i})
@SP
M=M+1"
            ),
            Self::Lt(i) => format!(
                "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@LOGICAL_LT_BODY_{i}
D;JLE

// (IF-else) -> no less than found
@SP
A=M
M=-1
@LOGICAL_LT_END_{i}
0;JMP

// (IF-then) it's less than or equal to zero
(LOGICAL_LT_BODY_{i})
@SP
A=M
M=0

(LOGICAL_LT_END_{i})
@SP
M=M+1"
            ),
            Self::And => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D&M
@SP
M=M+1"
                .into(),
            Self::Or => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D|M
@SP
M=M+1"
                .into(),
            Self::Not => "\
@SP
M=M-1
A=M
M=!M
@SP
M=M+1"
                .into(),
        };

        write!(f, "{}", s)
    }
}
