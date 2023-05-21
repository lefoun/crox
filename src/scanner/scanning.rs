use itertools::Itertools;

use super::token::{Token, TokenType};

pub struct Scanner {
    s: String,
}

impl<'a> Scanner {
    pub fn new(source: String) -> Self {
        Self { s: source }
    }

    pub fn tokenize(&'a self) -> Vec<Token<'a>> {
        use TokenType::*;

        // I've chosen to divide the length of the source code by 3
        // as a heuristic to allocate enough memory
        let mut tokens: Vec<Token> = Vec::with_capacity(self.s.len() / 3);
        let mut source = self.s.char_indices().peekable();
        let mut line = 0;

        while let Some((i, c)) = source.next() {
            match c {
                // Single character tokens
                '!' => {
                    if source.next_if_eq(&(i + 1, '=')).is_some() {
                        tokens.push(Token::new(BangEq, &self.s[i..i + 2], line))
                    } else {
                        tokens.push(Token::new(Bang, &self.s[i..i + 1], line))
                    }
                }
                '^' => tokens.push(Token::new(Carrot, &self.s[i..i + 1], line)),
                ',' => tokens.push(Token::new(Comma, &self.s[i..i + 1], line)),
                '.' => tokens.push(Token::new(Dot, &self.s[i..i + 1], line)),

                '=' => {
                    if source.next_if_eq(&(i + 1, '=')).is_some() {
                        tokens.push(Token::new(DoubleEq, &self.s[i..i + 2], line))
                    } else {
                        tokens.push(Token::new(Equal, &self.s[i..i + 1], line))
                    }
                }
                '>' => {
                    if source.next_if_eq(&(i + 1, '=')).is_some() {
                        tokens.push(Token::new(GreaterEq, &self.s[i..i + 2], line))
                    } else {
                        tokens.push(Token::new(Greater, &self.s[i..i + 1], line))
                    }
                }
                '{' => tokens.push(Token::new(LeftBrace, &self.s[i..i + 1], line)),
                '[' => tokens.push(Token::new(LeftBracket, &self.s[i..i + 1], line)),
                '(' => tokens.push(Token::new(LeftParen, &self.s[i..i + 1], line)),
                '<' => {
                    if source.next_if_eq(&(i + 1, '=')).is_some() {
                        tokens.push(Token::new(LessEq, &self.s[i..i + 2], line))
                    } else {
                        tokens.push(Token::new(Less, &self.s[i..i + 1], line))
                    }
                }
                '-' => tokens.push(Token::new(Minus, &self.s[i..i + 1], line)),
                '%' => tokens.push(Token::new(Percent, &self.s[i..i + 1], line)),
                '+' => tokens.push(Token::new(Plus, &self.s[i..i + 1], line)),
                ')' => tokens.push(Token::new(RightParen, &self.s[i..i + 1], line)),
                ']' => tokens.push(Token::new(RightBracket, &self.s[i..i + 1], line)),
                '}' => tokens.push(Token::new(RightBrace, &self.s[i..i + 1], line)),
                '*' => tokens.push(Token::new(Star, &self.s[i..i + 1], line)),
                ';' => tokens.push(Token::new(SemiColon, &self.s[i..i + 1], line)),
                ':' => tokens.push(Token::new(Colon, &self.s[i..i + 1], line)),

                // Two character tokens,
                '/' => {
                    if source.next_if_eq(&(i + 1, '/')).is_some() {
                        // We skip the comment line
                        if source.by_ref().any(|(_, chr)| chr == '\n') {
                            line += 1;
                        }
                    } else {
                        tokens.push(Token::new(Slash, &self.s[i..i + 1], line))
                    }
                }
                '0'..='9' => {
                    let curr = i
                        + 1
                        + source
                            .peeking_take_while(|(_, next_c)| next_c.is_numeric() || *next_c == '.')
                            .count();
                    if let Ok(num) = str::parse::<f64>(&self.s[i..curr]) {
                        tokens.push(Token::new(Number(num), &self.s[i..curr], line))
                    } else {
                        tokens.push(Token::new(
                            Error("Invalid Float literal"),
                            &self.s[i..curr],
                            line,
                        ))
                    }
                }
                'A'..='Z' | 'a'..='z' | '_' => {
                    let curr = i
                        + 1
                        + source
                            .peeking_take_while(|(_, next_c)| {
                                next_c.is_ascii_alphabetic() || *next_c == '_'
                            })
                            .count();
                    let lexeme = &self.s[i..curr];
                    tokens.push(Token::new(
                        Self::get_ident_or_keyword_token_ty(lexeme, c),
                        lexeme,
                        line,
                    ))
                }
                '"' => {
                    let mut curr = i + 1;
                    source
                        .peeking_take_while(|(_, next_c)| *next_c != '"')
                        .for_each(|(_, next_c)| {
                            if next_c == '\n' {
                                line += 1
                            }
                            curr += 1;
                        });
                    if let Some(&(_, '"')) = source.peek() {
                        source.next();
                        tokens.push(Token::new(CroxStr, &self.s[i + 1..curr], line));
                    } else {
                        tokens.push(Token::new(
                            Error("Unterminated String"),
                            &self.s[i + 1..curr],
                            line,
                        ))
                    }
                }
                ' ' => (),
                '\n' | '\r' | '\t' => line += 1,
                _ => tokens.push(Token::new(
                    Error("Unrecognized character"),
                    &self.s[i..i + 1],
                    line,
                )),
            }
        }
        tokens.push(Token::new(Eof, "", line));
        tokens
    }

    fn get_ident_or_keyword_token_ty(lexeme: &str, c: char) -> TokenType {
        use TokenType::*;
        match c {
            'a' if lexeme == "and" => And,
            'c' if lexeme == "class" => Class,
            'e' if lexeme == "else" => Else,
            'f' if lexeme == "false" => False,
            'f' if lexeme == "fn" => Fn,
            'f' if lexeme == "for " => For,
            'i' if lexeme == "if" => If,
            'n' if lexeme == "null" => Null,
            'o' if lexeme == "or" => Or,
            'r' if lexeme == "return" => Return,
            's' if lexeme == "super" => Super,
            't' if lexeme == "this" => This,
            't' if lexeme == "true" => True,
            'l' if lexeme == "let" => Let,
            'w' if lexeme == "while" => While,
            _ => Identifier,
        }
    }
}
