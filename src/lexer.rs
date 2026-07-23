//use std::env;
use crate::error::LexerError;
mod token;
use std::fs;
use std::path::Path;
use std::str;
use token::Token;

fn load(filename: &str) -> Result<String, LexerError> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(LexerError::FileNotFound {
            path: filename.to_string(),
        });
    };

    if path.extension().and_then(|extension| extension.to_str()) != Some("txt") {
        return Err(LexerError::InvalidExtension {
            error: "File must be a .txt file".to_string(),
        });
    };

    let contents = fs::read_to_string(&path).unwrap();
    Ok(contents)
}

fn tokenize(filename: &str) -> Result<Vec<Token>, LexerError> {
    let contents = load(filename)?;
    let mut tokens = Vec::new();
    for line in contents.lines() {
        match line.chars().next() {
            Some('.') => {
                block(&line, &mut tokens);
            }
            _ => {
                tokens.push(Token::NoInit);
                tokens.push(Token::Text(line.to_string()));
            }
        };
    }
    Ok(tokens)
}

fn block(line: &str, tokens: &mut Vec<Token>) {
    if line.starts_with(".hh ") {
        tokens.push(Token::HeaderInit);
        tokens.push(Token::Text(line[4..].to_string()));
    } else if line.starts_with(".h ") {
        tokens.push(Token::SubheaderInit);
        tokens.push(Token::Text(line[3..].to_string()));
    } else if line.starts_with(". ") {
        tokens.push(Token::ContentInit);
        tokens.push(Token::Text(line[2..].to_string()));
    } else {
        tokens.push(Token::NoInit);
        tokens.push(Token::Text(line.to_string()));
    }
}

pub fn lex(filename: &str) -> Result<Vec<Token>, LexerError> {
    tokenize(filename)
}
