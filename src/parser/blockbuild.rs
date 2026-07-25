use crate::lexer::token::Token;
use crate::parser::tempblock::ContentBlock;
use crate::parser::tempblock::HeaderBlock;
use crate::parser::tempblock::NoBlock;
use crate::parser::tempblock::TempBlock;

pub(super) fn build_block(tokens: Vec<Token>) -> Vec<TempBlock> {
    let mut blocks = Vec::new();
    let mut current_block = TempBlock::Header(HeaderBlock::default());
    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            Token::HeaderInit(num) => {
                blocks.push(current_block);
                current_block = TempBlock::Header(HeaderBlock::new(num));
            }
            Token::ContentInit => {
                blocks.push(current_block);
                current_block = TempBlock::Content(ContentBlock::default());
            }
            Token::NoInit => {
                blocks.push(current_block);
                current_block = TempBlock::None(NoBlock::default());
            }

            _ => {
                current_block.push(token);
            }
        }
    }
    blocks.push(current_block);
    blocks
}
