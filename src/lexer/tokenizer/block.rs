use crate::lexer::token::Token;

pub(super) fn block(line: &str, tokens: &mut Vec<Token>) {
    if line.starts_with(".h") {
        let mut level: String = String::new();
        for char in line[2..].chars() {
            if char.is_ascii_digit() {
                level.push(char);
            } else {
                break;
            }
        }
        match level.parse::<usize>() {
            Ok(level) => {
                if level == 0 {
                    let level = 1;
                    tokens.push(Token::HeaderInit(level));
                } else {
                    tokens.push(Token::HeaderInit(level));
                }
            }
            Err(_) => tokens.push(Token::HeaderInit(1)),
        }
        inline(&line[4..], tokens);
    } else if line.starts_with(".") {
        tokens.push(Token::ContentInit);
        inline(&line[2..], tokens);
    } else {
        tokens.push(Token::NoInit);
        tokens.push(Token::Newline);
        inline(&line, tokens);
    }
    tokens.push(Token::Newline);
}

pub(super) fn inline(line: &str, tokens: &mut Vec<Token>) {
    let mut buffer = String::new();
    for char in line.chars() {
        match char {
            '<' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(std::mem::take(&mut buffer)));
                }
                tokens.push(Token::BoldInit);
            }
            '>' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(std::mem::take(&mut buffer)));
                }
                tokens.push(Token::BoldEnd);
            }
            '[' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(std::mem::take(&mut buffer)));
                }
                tokens.push(Token::ItalicInit);
            }
            ']' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(std::mem::take(&mut buffer)));
                }
                tokens.push(Token::ItalicEnd);
            }

            _ => buffer.push(char),
        }
    }
    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer));
    }
}
