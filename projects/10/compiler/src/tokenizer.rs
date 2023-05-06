use crate::error;
use crate::token::{Token, TokenKind, TokenType};

const KEYWORDS: &[(&str, TokenType)] = &[
    ("class", TokenType::Class),
    ("constructor", TokenType::Constructor),
    ("function", TokenType::Function),
    ("method", TokenType::Method),
    ("field", TokenType::Field),
    ("static", TokenType::Static),
    ("var", TokenType::Var),
    ("int", TokenType::Int),
    ("char", TokenType::Char),
    ("boolean", TokenType::Boolean),
    ("void", TokenType::Void),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("null", TokenType::Null),
    ("this", TokenType::This),
    ("let", TokenType::Let),
    ("do", TokenType::Do),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("while", TokenType::While),
    ("return", TokenType::Return),
];

pub struct Tokenizer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }

    fn scan_token(&mut self) {
        let (kind, ty) = match self.advance() {
            '{' => (TokenKind::Symbol, TokenType::LeftBrace),
            '}' => (TokenKind::Symbol, TokenType::RightBrace),
            '(' => (TokenKind::Symbol, TokenType::LeftParen),
            ')' => (TokenKind::Symbol, TokenType::RightParen),
            '[' => (TokenKind::Symbol, TokenType::LeftBracket),
            ']' => (TokenKind::Symbol, TokenType::RightBracket),
            '.' => (TokenKind::Symbol, TokenType::Dot),
            ',' => (TokenKind::Symbol, TokenType::Comma),
            ';' => (TokenKind::Symbol, TokenType::Semicolon),
            '+' => (TokenKind::Symbol, TokenType::Plus),
            '-' => (TokenKind::Symbol, TokenType::Minus),
            '*' => (TokenKind::Symbol, TokenType::Star),
            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return;
                } else if self.match_('*') {
                    self.advance();

                    loop {
                        if self.is_at_end() {
                            break;
                        }

                        let c = self.advance();

                        if c == '\n' {
                            self.line += 1;
                        } else if c == '*' && self.peek() == '/' {
                            self.advance();
                            return;
                        }
                        // TODO: handle nested multi-line comments
                    }

                    self.error("Unterminated multi-line comment.");
                } else {
                    (TokenKind::Symbol, TokenType::Slash)
                }
            }
            '&' => (TokenKind::Symbol, TokenType::Ampersand),
            '|' => (TokenKind::Symbol, TokenType::Pipe),
            '<' => (TokenKind::Symbol, TokenType::Less),
            '>' => (TokenKind::Symbol, TokenType::Greater),
            '=' => (TokenKind::Symbol, TokenType::Equal),
            '~' => (TokenKind::Symbol, TokenType::Tilde),
            // Ignore whitespace
            ' ' | '\r' | '\t' => {
                return;
            }
            '\n' => {
                self.line += 1;
                return;
            }
            '"' => (TokenKind::StringConst, TokenType::String(self.string())),
            c => {
                if self.is_digit(c) {
                    (TokenKind::IntConst, TokenType::Number(self.number()))
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    self.error("Unexpected character.")
                }
            }
        };

        self.add_token(kind, ty);
    }

    fn string(&mut self) -> String {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
        }

        self.advance();

        (&self.source[self.start + 1..self.current - 1]).into()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> i32 {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        self.source[self.start..self.current].parse().unwrap()
    }

    fn identifier(&mut self) -> (TokenKind, TokenType) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        match KEYWORDS.iter().find(|(k, _)| k == &text) {
            Some((_, ty)) => (TokenKind::Keyword, ty.clone()),
            None => (
                TokenKind::Identifier,
                TokenType::Identifier(text.to_string()),
            ),
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, kind: TokenKind, ty: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(kind, ty, text.into(), self.line));
    }

    fn error(&self, msg: &str) -> ! {
        error::error(self.line, msg)
    }
}

#[test]
fn test_boolean() {
    let source = "
    true;  // Not false.
    false; // Not *not* false.
    ";
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenKind::Keyword, TokenType::True, "true".into(), 2),
            Token::new(TokenKind::Symbol, TokenType::Semicolon, ";".into(), 2),
            Token::new(TokenKind::Keyword, TokenType::False, "false".into(), 3),
            Token::new(TokenKind::Symbol, TokenType::Semicolon, ";".into(), 3),
        ]
    );
}

#[test]
fn test_multi_line_comment() {
    let source = "
    true;  /* aaa */
    /*
    false; // Not *not* false.
    */
    banana;
    ";
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.scan_tokens();
    assert_eq!(
        tokens,
        &vec![
            Token::new(TokenKind::Keyword, TokenType::True, "true".into(), 2),
            Token::new(TokenKind::Symbol, TokenType::Semicolon, ";".into(), 2),
            Token::new(
                TokenKind::Identifier,
                TokenType::Identifier("banana".into()),
                "banana".into(),
                5
            ),
            Token::new(TokenKind::Symbol, TokenType::Semicolon, ";".into(), 5),
        ]
    );
}
