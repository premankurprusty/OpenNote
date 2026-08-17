mod ast;
mod error;
mod lexer;
mod parser;
mod renderer;
use std::env;

use crate::error::LexerError;
fn main() {
    let filename = env::args_os().nth(1).unwrap();
    let tokens = lexer::lex(filename.to_str().unwrap());
    let valid_tokens;
    match tokens {
        Ok(value) => valid_tokens = value,
        Err(lexer_error) => match lexer_error {
            LexerError::FileNotFound { path } => {
                panic!("file: {} was not found", path);
            }
            LexerError::InvalidExtension { error } => {
                panic!("{}", error);
            }
        },
    }
    let document = parser::parse(valid_tokens);
    let output = renderer::render(document);
    println!("{}", output);
    _ = std::fs::write("output.html", output);
}
