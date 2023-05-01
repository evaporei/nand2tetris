use crate::instr::{Instr, Segment};
use std::str::Lines;

#[derive(Default)]
struct State {
    bool_count: usize,
    call_count: usize,
    curr_func: String,
}

pub struct Parser;

impl Parser {
    pub fn parse(lines: Lines) -> Vec<Instr> {
        lines
            .fold((State::default(), vec![]), |(state, mut instrs), line| {
                let (new_state, instr) = Self::parse_line(state, line);

                if let Some(instr) = instr {
                    instrs.push(instr);
                }

                (new_state, instrs)
            })
            .1
    }

    fn parse_line(mut state: State, line: &str) -> (State, Option<Instr>) {
        let trimmed = line.trim().chars();
        let chars: String = trimmed.take_while(|&ch| ch != '/').collect();
        let mut splitted = chars.split_whitespace();

        let instr = match splitted.next() {
            Some("push") => {
                let segment = match splitted.next() {
                    Some("constant") => Segment::Const,
                    Some("local") => Segment::Local,
                    Some("argument") => Segment::Argument,
                    Some("this") => Segment::This,
                    Some("that") => Segment::That,
                    Some("temp") => Segment::Temp,
                    Some("pointer") => Segment::Pointer,
                    Some("static") => Segment::Static,
                    Some(unknown) => panic!("unknown segment for push {unknown}"),
                    None => panic!("missing push first argument"),
                };

                let i = match splitted.next() {
                    Some(i) => {
                        let i = i.parse::<u16>().expect("second pop arg should be u16");

                        if segment == Segment::Pointer && i != 0 && i != 1 {
                            panic!("push pointer second argument must be 0 or 1")
                        }

                        i
                    }
                    None => panic!("missing push second argument"),
                };

                Some(Instr::Push(segment, i))
            }
            Some("pop") => {
                let segment = match splitted.next() {
                    Some("constant") => panic!("constant segment doesn't exist for pop"),
                    Some("local") => Segment::Local,
                    Some("argument") => Segment::Argument,
                    Some("this") => Segment::This,
                    Some("that") => Segment::That,
                    Some("temp") => Segment::Temp,
                    Some("pointer") => Segment::Pointer,
                    Some("static") => Segment::Static,
                    Some(unknown) => panic!("unknown segmentfor pop: {unknown}"),
                    None => panic!("missing pop first argument"),
                };

                let i = match splitted.next() {
                    Some(i) => {
                        let i = i.parse::<u16>().expect("second pop arg should be u16");

                        if segment == Segment::Pointer && i != 0 && i != 1 {
                            panic!("pop pointer second argument must be 0 or 1")
                        }

                        i
                    }
                    None => panic!("missing pop second argument"),
                };

                Some(Instr::Pop(segment, i))
            }
            Some("add") => Some(Instr::Add),
            Some("sub") => Some(Instr::Sub),
            Some("neg") => Some(Instr::Neg),
            Some("eq") => {
                let eq = Some(Instr::Eq(state.bool_count));
                state.bool_count += 1;
                eq
            }
            Some("gt") => {
                let gt = Some(Instr::Gt(state.bool_count));
                state.bool_count += 1;
                gt
            }
            Some("lt") => {
                let lt = Some(Instr::Lt(state.bool_count));
                state.bool_count += 1;
                lt
            }
            Some("and") => Some(Instr::And),
            Some("or") => Some(Instr::Or),
            Some("not") => Some(Instr::Not),
            Some("label") => {
                let label = splitted
                    .next()
                    .expect("missing label name argument in label command");

                let fmt_label = format!("({}{label})", state.curr_func);

                Some(Instr::Label(fmt_label))
            }
            Some("goto") => {
                let label = splitted
                    .next()
                    .expect("missing label name argument in goto command");

                let fmt_label = format!("{}{label}", state.curr_func);

                Some(Instr::Goto(fmt_label))
            }
            Some("if-goto") => {
                let label = splitted
                    .next()
                    .expect("missing label name argument in if-goto command");

                let fmt_label = format!("{}{label}", state.curr_func);

                Some(Instr::IfGoto(fmt_label))
            }
            Some("function") => {
                let name = splitted
                    .next()
                    .expect("missing name argument in function command")
                    .to_string();

                let n_vars = splitted
                    .next()
                    .expect("missing nVars argument in function command")
                    .parse()
                    .expect("nVars argument in function should be a positive integer");

                state.curr_func = format!("{name}$");

                Some(Instr::Function(name, n_vars))
            }
            Some("return") => Some(Instr::Return),
            Some("call") => {
                let name = splitted
                    .next()
                    .expect("missing name argument in call command")
                    .to_string();

                let n_args = splitted
                    .next()
                    .expect("missing nArgs argument in call command")
                    .parse()
                    .expect("nArgs argument in call should be a positive integer");

                let call = Some(Instr::Call(name, n_args, state.call_count));
                state.call_count += 1;
                call
            }
            Some(s) => {
                if s.starts_with("/") {
                    None
                } else {
                    panic!("kaboom {s}")
                }
            }
            None => None,
        };

        (state, instr)
    }
}
