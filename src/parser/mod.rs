mod blockbuild;
mod cleaner;
mod inlinebuild;
mod tempblock;
mod tempinlined;

use crate::{ast::Inline, lexer::token::Token};
use blockbuild::build_block;
use cleaner::clean;
use inlinebuild::build_inline;
use tempinlined::TempInlined;

pub fn parse(tokens: Vec<Token>) -> Vec<TempInlined> {
    let blocks = build_block(tokens);
    println!("{:#?}", blocks);
    let parsed_blocks = build_inline(blocks);
    println!("{:#?}", parsed_blocks);
    let cleaned_blocks = clean(parsed_blocks);
    cleaned_blocks
}
