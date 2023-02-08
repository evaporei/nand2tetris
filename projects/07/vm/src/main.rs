use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use vm::parser::Parser;

/// Removes extension from file name.
fn file_name(file: &str) -> Option<String> {
    Path::new(file)
        .file_name()
        .and_then(OsStr::to_str)
        .map(|name| name.chars().take_while(|ch| *ch != '.').collect())
}

/// The vm converts the vm byte code into Hack Assembly
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file = &args[0];
    let program = fs::read_to_string(file.clone()).expect("vm program doesn't exist");

    let parser = Parser::new(program.lines());

    let instructions: Vec<String> = parser.parse();

    let mut translated_file = File::create(format!("{}.asm", file_name(&file).unwrap()))
        .expect("failed to create assembly file");

    for instruction in instructions {
        translated_file
            .write_all(instruction.as_bytes())
            .expect("failed to write instruction to translated file");

        translated_file
            .write_all(b"\n")
            .expect("failed to write new line to translated file");
    }
}
