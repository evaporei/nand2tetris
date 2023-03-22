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
    Return,
    Call(String, usize),
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
    pub fn to_assembly(self, file_name: &str) -> ASMCode {
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
            Self::Function(name, n_vars) => {
                let label = format!("({name})");

                let mut pushes = String::new();
                for _ in 0..n_vars {
                    pushes.push_str(&Self::Push(Segment::Const, 0).to_assembly(file_name));
                    pushes.push('\n');
                }

                format!("{label}\n{pushes}")
            }
            Self::Return => {
                // R14=FRAME
                let save_frame = "\
@LCL
D=M
@R14
M=D";

                // R15=RET_ADDR
                let save_return_addr = "\
@5
A=D-A
D=M
@R15
M=D";

                let return_val = Self::Pop(Segment::Argument, 0).to_assembly(&file_name);

                let adjust_sp = "\
@ARG
D=M
@SP
M=D+1";

                // AMD=M-1 is equivalent to D=M-1 and AM=D
                let restore_that = "\
@R14
AMD=M-1
D=M
@THAT
M=D";

                let restore_this = "\
@R14
AMD=M-1
D=M
@THIS
M=D";

                let restore_arg = "\
@R14
AMD=M-1
D=M
@ARG
M=D";

                let restore_lcl = "\
@R14
AMD=M-1
D=M
@LCL
M=D";

                let goto_ret_addr = "\
@R15
A=M
0;JMP";

                format!("{save_frame}\n{save_return_addr}\n{return_val}\n{adjust_sp}\n{restore_that}\n{restore_this}\n{restore_arg}\n{restore_lcl}\n{goto_ret_addr}")
            }
            Self::Call(name, n_args) => {
                let push_const = |addr: &str, where_: &str| {
                    format!(
                        "\
@{addr}
D={where_}
@SP
A=M
M=D
@SP
M=M+1"
                    )
                };

                let push_label_addr = push_const(&format!("CALL_{name}"), "A");

                let push_lcl = push_const("LCL", "M");
                let push_arg = push_const("ARG", "M");
                let push_this = push_const("THIS", "M");
                let push_that = push_const("THAT", "M");

                // ARG = SP - 5 - nArgs
                let reposition_arg = format!(
                    "\
@SP
D=M
@5
D=D-A
@{n_args}
D=D-A"
                );

                // LCL = SP
                let reposition_lcl = "\
@SP
D=M
@LCL
M=D";

                let jmp_to_fn = format!(
                    "\
@{name}
0;JMP"
                );

                let label = format!("(CALL_{name})");

                format!("{push_label_addr}\n{push_lcl}\n{push_arg}\n{push_this}\n{push_that}\n{reposition_arg}\n{reposition_lcl}\n{jmp_to_fn}\n{label}")
            }
        }
    }
}
