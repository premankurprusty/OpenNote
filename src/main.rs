mod ast;
mod error;
mod lexer;
mod parser;
use std::env;
fn main() {
    let filename = env::args_os().nth(1).unwrap();
    let tokens = lexer::lex(filename.to_str().unwrap()).unwrap();
    let tokens = parser::parse(tokens);
    for token in tokens {
        println!("{:#?}", token);
    }
}
