use crate::lexer::token::Token;

pub(super) fn block(line: &str, tokens: &mut Vec<Token>) {
    if line.starts_with(".hh ") {
        tokens.push(Token::HeaderInit);
        inline(&line[4..], tokens);
    } else if line.starts_with(".h ") {
        tokens.push(Token::SubheaderInit);
        inline(&line[3..], tokens);
    } else if line.starts_with(". ") {
        tokens.push(Token::ContentInit);
        inline(&line[2..], tokens);
    } else {
        tokens.push(Token::NoInit);
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
