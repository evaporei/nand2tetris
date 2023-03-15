pub enum Instr {
    Push(Segment, u16),
    Pop(Segment, u16),
    Add,
    Sub,
    Neg,
    Eq(usize),
    Gt(usize),
    Lt(usize),
    And,
    Or,
    Not,
    Label(String),
    Goto(String),
    IfGoto(String),
    Function(String, usize),
}

#[derive(PartialEq)]
pub enum Segment {
    Const,
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static,
}

pub type ASMCode = String;

impl Instr {
    pub fn to_assembly(self, file_name: String) -> ASMCode {
        match self {
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
                Segment::Local => format!(
                    "\
@{i}
D=A
@LCL
A=M+D
D=M
@SP
M=M+1
A=M-1
M=D"
                ),
                Segment::Argument => format!(
                    "\
@{i}
D=A
@ARG
A=M+D
D=M
@SP
M=M+1
A=M-1
M=D"
                ),
                Segment::This => format!(
                    "\
@{i}
D=A
@THIS
A=M+D
D=M
@SP
M=M+1
A=M-1
M=D"
                ),
                Segment::That => format!(
                    "\
@{i}
D=A
@THAT
A=M+D
D=M
@SP
M=M+1
A=M-1
M=D"
                ),
                Segment::Temp => format!(
                    "\
@{i}
D=A
@5
A=D+A
D=M
@SP
M=M+1
A=M-1
M=D"
                ),
                Segment::Pointer => match i {
                    0 => format!(
                        "\
@THIS
D=M
@SP
M=M+1
A=M-1
M=D"
                    ),
                    1 => format!(
                        "\
@THAT
D=M
@SP
M=M+1
A=M-1
M=D"
                    ),
                    _ => unreachable!("push pointer only accepts 0 or 1 as argument"),
                },
                Segment::Static => format!(
                    "\
@{i}
D=A
@{file_name}.{i}
A=M+D
D=M
@SP
M=M+1
A=M-1
M=D"
                ),
            },
            Self::Pop(segment, i) => match segment {
                Segment::Const => unreachable!("there is no constant segment to pop"),
                Segment::Local => format!(
                    "\
@{i}
D=A
@LCL
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D"
                ),
                Segment::Argument => format!(
                    "\
@{i}
D=A
@ARG
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D"
                ),
                Segment::This => format!(
                    "\
@{i}
D=A
@THIS
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D"
                ),
                Segment::That => format!(
                    "\
@{i}
D=A
@THAT
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D"
                ),
                Segment::Temp => format!(
                    "\
@{i}
D=A
@5
D=D+A
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D"
                ),
                Segment::Pointer => match i {
                    0 => format!(
                        "\
@SP
M=M-1
A=M
D=M
@THIS
M=D"
                    ),
                    1 => format!(
                        "\
@SP
M=M-1
A=M
D=M
@THAT
M=D"
                    ),
                    _ => unreachable!("pop pointer only accepts 0 or 1 as argument"),
                },
                Segment::Static => format!(
                    "\
@{i}
D=A
@{file_name}.{i}
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D"
                ),
            },
            // arithmetic
            Self::Add => "\
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=D+M"
                .into(),
            Self::Sub => "\
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=M-D"
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
A=M-1
D=D-M
@EQ_BODY_{i}
D;JEQ

// (IF-else) -> no equality found
@SP
A=M-1
M=0
@EQ_END_{i}
0;JMP

// (IF-then) it's equal to zero, which means equality
(EQ_BODY_{i})
@SP
A=M-1
M=-1

(EQ_END_{i})"
            ),
            Self::Gt(i) => format!(
                "\
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=D-M
@GT_ELSE_{i}
D;JLT

// (IF-then) it's greater than or equal to zero
@SP
A=M-1
M=0
@GT_END_{i}
0;JMP

// (IF-else) -> no greater than found
(GT_ELSE_{i})
@SP
A=M-1
M=-1

(GT_END_{i})"
            ),
            Self::Lt(i) => format!(
                "\
@SP
M=M-1
A=M
D=M
@SP
A=M-1
D=D-M
@LT_ELSE_{i}
D;JGT

// (IF-then) it's less than or equal to zero
@SP
A=M-1
M=0
@LT_END_{i}
0;JMP

// (IF-else) -> no less than found
(LT_ELSE_{i})
@SP
A=M-1
M=-1

(LT_END_{i})"
            ),
            Self::And => "\
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=D&M"
                .into(),
            Self::Or => "\
@SP
M=M-1
A=M
D=M
@SP
A=M-1
M=D|M"
                .into(),
            Self::Not => "\
@SP
M=M-1
A=M
M=!M
@SP
M=M+1"
                .into(),
            Self::Label(label) => format!(
                "\
({file_name}.{label})"
            ),
            Self::Goto(label) => format!(
                "\
@{file_name}.{label}
0;JMP"
            ),
            Self::IfGoto(label) => format!(
                "\
@SP
M=M-1
A=M
D=M
@{file_name}.{label}
D;JNE"
            ),
            Self::Function(name, args) => format!(
                "\
"
            ),
        }
    }
}
