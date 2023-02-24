use crate::instr::{Instr, Segment};
use std::str::Lines;

pub struct Parser;

impl Parser {
    pub fn parse(lines: Lines) -> Vec<Instr> {
        lines
            .enumerate()
            .map(|(i, line)| Self::parse_line(i, line))
            .filter_map(|x| x)
            .collect()
    }

    fn parse_line(i: usize, line: &str) -> Option<Instr> {
        let trimmed = line.trim().chars();
        let chars: String = trimmed.take_while(|&ch| ch != '/').collect();
        let mut splitted = chars.split_whitespace();

        match splitted.next() {
            Some("push") => {
                let segment = match splitted.next() {
                    Some("constant") => Segment::Const,
                    Some("local") => Segment::Local,
                    Some("argument") => Segment::Argument,
                    Some("this") => Segment::This,
                    Some("that") => Segment::That,
                    Some("temp") => Segment::Temp,
                    Some(unknown) => panic!("unknown segment for push {unknown}"),
                    None => panic!("missing push first argument"),
                };

                let i = match splitted.next() {
                    Some(i) => i.parse::<u16>().expect("second push arg should be u16"),
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
                    Some(unknown) => panic!("unknown segmentfor pop: {unknown}"),
                    None => panic!("missing pop first argument"),
                };

                let i = match splitted.next() {
                    Some(i) => i.parse::<u16>().expect("second pop arg should be u16"),
                    None => panic!("missing pop second argument"),
                };

                Some(Instr::Pop(segment, i))
            }
            Some("add") => Some(Instr::Add),
            Some("sub") => Some(Instr::Sub),
            Some("neg") => Some(Instr::Neg),
            Some("eq") => Some(Instr::Eq(i)),
            Some("gt") => Some(Instr::Gt(i)),
            Some("lt") => Some(Instr::Lt(i)),
            Some("and") => Some(Instr::And),
            Some("or") => Some(Instr::Or),
            Some("not") => Some(Instr::Not),
            Some(s) => {
                if s.starts_with("/") {
                    None
                } else {
                    panic!("kaboom {s}")
                }
            }
            None => None,
        }
    }
}
