use regex::Regex;

mod token;
use token::{
    Token, 
    TokenType,
};

mod lexer;
use lexer::Lexer;

fn main() {
    //let reg = Regex::new(r"h.*").unwrap();
    //assert!(reg.is_match("pello"));
    let token = Token::new(TokenType::Identifier, "ass");
    println!("{:?}", token);

    let lexer = Lexer::init("test.goyo");
}

