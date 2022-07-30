use assembler::parser::Parser;
use std::fs::File;
use std::io::Write;
use std::{env, fs};

use std::ffi::OsStr;
use std::path::Path;

/// Removes extension from file name.
fn file_name(file: &str) -> Option<String> {
    Path::new(file)
        .file_name()
        .and_then(OsStr::to_str)
        .map(|name| name.chars().take_while(|ch| *ch != '.').collect())
}

/// The assembler converts the Hack computer machine language
/// from the text form, to the binary form.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file = &args[0];
    let program = fs::read_to_string(file.clone()).expect("hack program doesn't exist");

    let instructions = Parser::parse(program.lines());

    let mut translated_file = File::create(format!("{}.hack", file_name(&file).unwrap()))
        .expect("failed to create translated file");
    for instruction in instructions {
        translated_file
            .write_all(instruction.as_bytes())
            .expect("failed to write instruction to translated file");

        translated_file
            .write_all(b"\n")
            .expect("failed to write new line to translated file");
    }
}
