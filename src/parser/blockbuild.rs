use crate::lexer::token::Token;
use crate::parser::tmp_asts::Block;
use crate::parser::tmp_asts::ContentBlock;
use crate::parser::tmp_asts::HeaderBlock;
use crate::parser::tmp_asts::NoBlock;

pub fn build_block(tokens: Vec<Token>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current_block = Block::Header(HeaderBlock::default());
    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            Token::HeaderInit(num) => {
                blocks.push(current_block);
                current_block = Block::Header(HeaderBlock::new(num));
            }
            Token::ContentInit => {
                blocks.push(current_block);
                current_block = Block::Content(ContentBlock::default());
            }
            Token::NoInit => {
                blocks.push(current_block);
                current_block = Block::None(NoBlock::default());
            }

            _ => {
                current_block.push(token);
            }
        }
    }
    blocks.push(current_block);
    blocks
}
