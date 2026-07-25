use crate::ast::Inline;
use crate::lexer::token::Token;
use crate::parser::tempblock::TempBlock;
use crate::parser::tempinlined::TempInlined;

pub(super) fn build_inline(blocks: Vec<TempBlock>) -> Vec<TempInlined> {
    let mut result = Vec::new();
    for block in blocks {
        let mut inlined = Vec::new();
        let mut contents = block.contents().into_iter().peekable();
        while let Some(token) = contents.next() {
            match token {
                Token::Text(text) => inlined.push(Inline::Text(text)),
                Token::BoldInit => inlined.push(Inline::Bold(build_bold(&mut contents))),
                Token::BoldEnd => inlined.push(Inline::Text(">".to_string())),
                Token::ItalicInit => inlined.push(Inline::Italic(build_italic(&mut contents))),
                Token::ItalicEnd => inlined.push(Inline::Text("]".to_string())),
                Token::Newline => inlined.push(Inline::NewLine),
                Token::HeaderInit(_) | Token::ContentInit | Token::NoInit => {}
            }
        }
        match block {
            TempBlock::Header(block) => result.push(TempInlined::Header(block.level, inlined)),
            TempBlock::Content(_) => result.push(TempInlined::Paragraph(inlined)),
            TempBlock::None(_) => result.push(TempInlined::Notype(inlined)),
        }
    }
    result
}

fn build_bold(contents: &mut std::iter::Peekable<impl Iterator<Item = Token>>) -> Vec<Inline> {
    let mut inlined = Vec::new();
    while let Some(token) = contents.next() {
        match token {
            Token::Text(text) => inlined.push(Inline::Text(text)),
            Token::BoldInit => inlined.push(Inline::Bold(build_bold(contents))),
            Token::BoldEnd => return inlined,
            Token::ItalicInit => inlined.push(Inline::Italic(build_italic(contents))),
            Token::ItalicEnd => {
                let mut temp = vec![Inline::ToBeCleaned("<".to_string())];
                temp.append(&mut inlined);
                temp.push(Inline::Text("]".to_string()));
                return temp;
            }
            Token::Newline => inlined.push(Inline::NewLine),
            Token::HeaderInit(_) | Token::ContentInit | Token::NoInit => {}
        }
    }
    let mut temp = vec![Inline::ToBeCleaned("<".to_string())];
    temp.append(&mut inlined);
    return temp;
}

fn build_italic(contents: &mut std::iter::Peekable<impl Iterator<Item = Token>>) -> Vec<Inline> {
    let mut inlined = Vec::new();
    while let Some(token) = contents.next() {
        match token {
            Token::Text(text) => inlined.push(Inline::Text(text)),
            Token::BoldInit => inlined.push(Inline::Bold(build_bold(contents))),
            Token::ItalicEnd => return inlined,
            Token::ItalicInit => inlined.push(Inline::Italic(build_italic(contents))),
            Token::BoldEnd => {
                let mut temp = vec![Inline::ToBeCleaned("[".to_string())];
                temp.append(&mut inlined);
                temp.push(Inline::Text(">".to_string()));
                return temp;
            }
            Token::Newline => inlined.push(Inline::NewLine),
            Token::HeaderInit(_) | Token::ContentInit | Token::NoInit => {}
        }
    }
    let mut temp = vec![Inline::ToBeCleaned("[".to_string())];
    temp.append(&mut inlined);
    return temp;
}
