use assembler::parser::Parser;
use std::{env, fs};

/// The assembler converts the Hack computer machine language
/// from the text form, to the binary form.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file_name = &args[0];
    let program = fs::read_to_string(file_name).expect("hack program doesn't exist");

    let _tokens = Parser::parse(program.lines());
}
