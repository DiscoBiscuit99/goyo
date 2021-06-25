use goyo::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::init("test.goyo");

    lexer.tokenize();

    for token in lexer.tokens {
        println!("{:?}", token);
    }


}

