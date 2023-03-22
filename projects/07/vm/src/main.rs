use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use vm::instr::Instr;
use vm::parser::Parser;

/// Removes extension from file name.
fn file_name(file: &str) -> Option<String> {
    Path::new(file)
        .file_name()
        .and_then(OsStr::to_str)
        .map(|name| name.chars().take_while(|ch| *ch != '.').collect())
}

fn compile(source: &str) -> Vec<Instr> {
    Parser::parse(source.lines())
}

fn create_asm_file(file: &str) -> File {
    let mut new_file = PathBuf::from(file);

    new_file.set_extension("asm");

    File::create(new_file).expect("failed to create assembly file")
}

fn write_asm(file: &mut File, file_name: &str, instructions: Vec<Instr>) {
    for instruction in instructions {
        file.write_all(instruction.to_assembly(&file_name).as_bytes())
            .expect("failed to write instruction to translated file");

        file.write_all(b"\n")
            .expect("failed to write new line to translated file");
    }
}

/// The vm converts the vm byte code into Hack Assembly
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file_or_dir = &args[0];
    let metadata = fs::metadata(file_or_dir).unwrap();

    if metadata.is_file() {
        let file = file_or_dir;

        let source = fs::read_to_string(file).expect("vm program doesn't exist");

        let instructions = compile(&source);

        let mut new_file = create_asm_file(file);

        write_asm(&mut new_file, &file_name(file).unwrap(), instructions);
    } else if metadata.is_dir() {
        let mut dirname = PathBuf::from(file_or_dir);

        dirname.push(file_name(file_or_dir).unwrap());

        let paths = fs::read_dir(file_or_dir).unwrap();

        let mut new_file = create_asm_file(&dirname.to_str().unwrap());

        for path in paths {
            let path = path.unwrap().path();
            if path.extension().map(OsStr::to_str).map(Option::unwrap) == Some("vm") {
                let source =
                    fs::read_to_string(path.to_str().unwrap()).expect("vm program doesn't exist");

                let instructions = compile(&source);

                write_asm(
                    &mut new_file,
                    path.file_name().unwrap().to_str().unwrap(),
                    instructions,
                );
            }
        }
    } else {
        panic!("unsupported parameter, maybe passing symlink?");
    }
}
