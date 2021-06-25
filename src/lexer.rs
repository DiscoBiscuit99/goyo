use std::fs;
use std::collections::HashMap;

use crate::token::{
    Token, TokenType,
};

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pub keywords: HashMap<String, TokenType>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn init(source_name: &str) -> Self {
        let source = fs::read_to_string(source_name).unwrap();

        let mut keywords = HashMap::new();

        keywords.insert(String::from("let"),   TokenType::Let);
        keywords.insert(String::from("func"),  TokenType::Func);
        keywords.insert(String::from("end"),   TokenType::End);
        keywords.insert(String::from("if"),    TokenType::If);
        keywords.insert(String::from("else"),  TokenType::Else);
        keywords.insert(String::from("true"),  TokenType::True);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"),   TokenType::For);
        keywords.insert(String::from("while"), TokenType::While);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("self"),  TokenType::Slf);

        Self { 
            source, 
            keywords,
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) {
        while !self.end_reached() {
            self.consume_token();
        }
    }

    pub fn consume_token(&mut self) {
        let c = self.consume_char();
        match c {
            // shorter lexemes
            '(' => self.add_token(TokenType::LeftParen,  c.to_string()),
            ')' => self.add_token(TokenType::RightParen, c.to_string()),
            ',' => self.add_token(TokenType::Comma,      c.to_string()),
            '.' => self.add_token(TokenType::Dot,        c.to_string()),
            '-' => self.add_token(TokenType::Minus,      c.to_string()),
            '+' => self.add_token(TokenType::Plus,       c.to_string()),
            ';' => self.add_token(TokenType::Semicolon,  c.to_string()),
            '*' => self.add_token(TokenType::Star,       c.to_string()),

            // longer lexemes
            ':' => {
                if self.peek() == '=' {
                    self.consume_char();
                    self.add_token(TokenType::ColonEqual, ":=".to_string());
                } else {
                    panic!("Unexpected token! `{}`", c)
                }
            },
            '|' => {
                if self.peek() == '|' {
                    self.consume_char();
                    self.add_token(TokenType::Or, "||".to_string());
                } else {
                    // bitwise operation
                    unimplemented!();
                }
            },
            '&' => {
                if self.peek() == '&' {
                    self.consume_char();
                    self.add_token(TokenType::And, "&&".to_string());
                } else {
                    // bitwise operation
                    unimplemented!();
                }
            }

            // strings
            '"' => {
                self.consume_string(c);
            }

            // comments: ignore everything after the '#'
            '#' => {
                while self.peek() != '\n' {
                    self.consume_char();
                }
            }
            
            // ignore whitespace
            ' ' => (),
            '\t' => (),
            '\r' => (),
            '\n' => (),

            // null -> end of file
            '\0' => self.add_token(TokenType::Eof, c.to_string()),

            // otherwise
            _ => {
                if c.is_numeric() {
                    self.consume_number(c);
                } else if c.is_alphabetic() {
                    self.consume_identifier(c);
                } else {
                    panic!("Unexpected character! `{}`", c) 
                }
            },
        }
    }

    fn consume_char(&mut self) -> char {
        if self.end_reached() { return '\0' }
        self.source.remove(0)
    }

    fn consume_number(&mut self, first_char: char) {
        let mut number = first_char.to_string();
        while self.peek().is_numeric() { number.push(self.consume_char()); }

        // look for fractional part
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // consume the '.'
            number.push(self.consume_char());

            while self.peek().is_numeric() { number.push(self.consume_char()); }
        }

        self.add_token(TokenType::Number, number);
    }

    fn consume_string(&mut self, first_char: char) {
        let mut string = first_char.to_string();

        while !self.end_reached() && self.peek() != '"' {
            string.push(self.consume_char());
        } 
        string.push(self.consume_char());

        self.add_token(TokenType::String, string);
    }

    fn consume_identifier(&mut self, first_char: char) {
        let mut identifier = first_char.to_string();
        while self.peek().is_alphanumeric() { identifier.push(self.consume_char()); } 
        let type_of_token: TokenType;

        if let Some(token_type) = self.keywords.get(&identifier) {
            type_of_token = *token_type;
        } else {
            type_of_token = TokenType::Identifier;
        }

        self.add_token(type_of_token, identifier);
    }

    fn peek(&self) -> char {
        if self.end_reached() { return '\0' }
        self.source.chars().next().unwrap()
    }

    fn peek_next(&self) -> char {
        if self.end_reached() { return '\0' }
        self.source.chars().nth(1).unwrap()
    }

    fn add_token(&mut self, type_: TokenType, literal: String) {
        self.tokens.push(Token::new(type_, literal));
    }

    // just a wrapper function
    fn end_reached(&self) -> bool {
        self.source.is_empty()
    }
}

