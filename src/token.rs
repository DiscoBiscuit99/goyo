use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // one or two character tokens
    LeftParen, RightParen,
    Comma, Dot, Minus, 
    Plus, Star, Pound, 
    Equal, Bang, Colon, 
    Semicolon, ColonEqual,

    // literals
    Identifier, Str, Number,

    // keywords (Slf := Self)
    Let, Func, End, And, 
    Or, If, Else, True, 
    False, For, While, 
    Print, Return, Slf,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    type_: TokenType,
    literal: String,
}

impl Token {
    pub fn new(type_: TokenType, literal: String) -> Self {
        Self { type_, literal }
    }
}

