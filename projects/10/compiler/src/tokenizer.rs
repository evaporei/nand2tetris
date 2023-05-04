use std::str::Lines;

pub struct Tokenizer<'a> {
    source: &'a str,
    tokens: Vec<String>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<String> {
        &self.tokens
    }
}
