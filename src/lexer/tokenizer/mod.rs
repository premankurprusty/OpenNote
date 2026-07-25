mod block;
use crate::lexer::tokenizer::block::block;
use crate::lexer::tokenizer::block::inline;

use crate::error::LexerError;
use crate::lexer::token::Token;

pub(super) fn tokenize(content: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    for line in content.lines() {
        if line.is_empty() {
            tokens.push(Token::Newline);
        } else {
            block(&line, &mut tokens);
        };
    }
    Ok(tokens)
}
