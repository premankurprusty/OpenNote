mod error;
mod lexer;
use std::env;
fn main() {
    let filename = env::args_os().nth(1).unwrap();
    let tokens = lexer::lex(filename.to_str().unwrap()).unwrap();
    for token in tokens {
        println!("{:?}", token);
    }
}
