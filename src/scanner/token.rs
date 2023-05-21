#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    // One character tokens,
    Bang,
    Carrot,
    Colon,
    Comma,
    Dot,
    Equal,
    Greater,
    LeftBrace,
    LeftBracket,
    LeftParen,
    Less,
    Minus,
    Percent,
    Plus,
    RightBrace,
    RightBracket,
    RightParen,
    Star,
    SemiColon,
    Slash,

    // Two character tokens,
    LessEq,
    GreaterEq,
    DoubleEq,
    BangEq,

    // Literals
    Identifier,
    Number(f64),
    CroxStr,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fn,
    If,
    Let,
    Null,
    Or,
    Return,
    Super,
    This,
    True,
    While,

    Eof,
    Error(&'static str),
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    ty: TokenType,
    lexeme: &'a str,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, lexeme: &'a str, line: usize) -> Self {
        Self { ty, lexeme, line }
    }
}
