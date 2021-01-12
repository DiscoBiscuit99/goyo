use std::fs;

use crate::token::{
    Token, TokenType,
};

pub struct Lexer {
    pub source: String,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn init(source_name: &str) -> Self {
        let source = fs::read_to_string(source_name).unwrap();
        let tokens = Vec::new();

        Self { source, tokens }
    }

    pub fn consume_char(&mut self) -> Option<char> {
        match self.peek() {
            Some(c) => {
                Some(self.source.remove(0))
            },
            None => None,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source.chars().next()
    }
}

