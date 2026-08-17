mod ast;
mod error;
mod lexer;
mod parser;
mod renderer;
use std::env;
fn main() {
    let filename = env::args_os().nth(1).unwrap();
    let tokens = lexer::lex(filename.to_str().unwrap()).unwrap();
    let tokens = parser::parse(tokens);
    let output = renderer::render(tokens);
    println!("{}", output);
    _ = std::fs::write("output.html", output);
}
