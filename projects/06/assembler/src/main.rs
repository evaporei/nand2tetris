use assembler::parser::Parser;
use std::fs::File;
use std::io::Write;
use std::{env, fs};

use std::path::PathBuf;

/// The assembler converts the Hack computer machine language
/// from the text form, to the binary form.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file = &args[0];
    let program = fs::read_to_string(file.clone()).expect("hack program doesn't exist");

    let (lines, symbol_table) = Parser::first_pass(program.lines());

    let instructions = Parser::second_pass(lines, symbol_table);

    let mut new_file = PathBuf::from(file);

    new_file.set_extension("hack");

    let mut translated_file = File::create(new_file).expect("failed to create translated file");

    for instruction in instructions {
        translated_file
            .write_all(instruction.as_bytes())
            .expect("failed to write instruction to translated file");

        translated_file
            .write_all(b"\n")
            .expect("failed to write new line to translated file");
    }
}
