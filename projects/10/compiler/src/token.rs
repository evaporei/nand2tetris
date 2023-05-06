#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // symbol
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Dot,          // .
    Comma,        // ,
    Semicolon,    // ;
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Ampersand,    // &
    Pipe,         // |
    Less,         // <
    Greater,      // >
    Equal,        // =
    Tilde,        // ~

    // stringConstant
    String(String),

    // integerConstant
    Number(i32),

    // identifier
    Identifier(String),

    // Keywords
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Symbol,
    StringConst,
    IntConst,
    Identifier,
    Keyword,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    kind: TokenKind,
    #[allow(dead_code)]
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, ty: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            kind,
            ty,
            lexeme,
            line,
        }
    }

    pub fn to_xml_tag(&self) -> String {
        let tag = match self.kind {
            TokenKind::Symbol => "symbol",
            TokenKind::StringConst => "stringConstant",
            TokenKind::IntConst => "integerConstant",
            TokenKind::Identifier => "identifier",
            TokenKind::Keyword => "keyword",
        };

        let mut value = &self.lexeme;

        if let TokenType::String(v) = &self.ty {
            value = &v; // value without enclosing ""
        }

        let value = value
            .replace("&", "&amp;") // has to be first
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;");

        format!("<{tag}> {value} </{tag}>")
    }
}
