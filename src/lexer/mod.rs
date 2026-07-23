mod loader;
mod token;
mod tokenizer;
use crate::error::LexerError;
//use crate::lexer::token::Token;
use loader::load;
use tokenizer::tokenize;

pub fn lex(filename: &str) -> Result<Vec<token::Token>, LexerError> {
    let contents = load(filename)?;
    tokenize(&contents)
}
