use compiler::tokenizer::Tokenizer;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

fn create_tokens_file(file: &str) -> File {
    let mut new_file = PathBuf::from(file);

    let name_with_t = format!("{}T", new_file.file_stem().unwrap().to_str().unwrap());

    new_file.set_file_name(name_with_t);
    new_file.set_extension("xml");

    File::create(new_file).expect("failed to create assembly file")
}

fn write_tokens(file: &mut File, tokens: &Vec<String>) {
    for token in tokens {
        file.write_all(token.as_bytes())
            .expect("failed to write token to tokens file");

        file.write_all(b"\n")
            .expect("failed to write new line to tokens file");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let file_or_dir = &args[0];
    let metadata = fs::metadata(file_or_dir).unwrap();

    if metadata.is_file() {
        let file = file_or_dir;

        let source = fs::read_to_string(file).expect("vm program doesn't exist");

        let mut tokenizer = Tokenizer::new(&source);

        let tokens = tokenizer.scan_tokens();

        let mut new_file = create_tokens_file(file);

        write_tokens(&mut new_file, tokens);
    } else if metadata.is_dir() {
        panic!("directories not supported yet");
    } else {
        panic!("unsupported parameter, maybe passing symlink?");
    }
}
