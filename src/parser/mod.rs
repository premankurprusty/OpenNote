mod blockbuild;
mod tmp_asts;
use crate::lexer::token::Token;
pub use blockbuild::build_block;
use tmp_asts::Block;

pub fn parse(tokens: Vec<Token>) -> Vec<Block> {
    build_block(tokens)
}
