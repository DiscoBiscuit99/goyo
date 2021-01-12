use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    Let, Assign, Semicolon,
    Identifier, Func, End,
}

#[derive(Debug)]
pub struct Token {
    type_: TokenType,
    literal: String,
}

impl Token {
    pub fn new(type_: TokenType, literal: &str) -> Self {
        Self { type_, literal: literal.to_string() }
    }
}

