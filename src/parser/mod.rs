mod blockbuild;
mod cleaner;
mod finalbuild;
mod inlinebuild;
mod tempblock;
mod tempinlined;

use crate::{ast::Document, lexer::token::Token};
use blockbuild::build_block;
use cleaner::clean;
use finalbuild::build_final;
use inlinebuild::build_inline;

pub fn parse(tokens: Vec<Token>) -> Document {
    let blocks = build_block(tokens);
    let parsed_blocks = build_inline(blocks);
    let cleaned_blocks = clean(parsed_blocks);
    let ast = build_final(cleaned_blocks);
    ast
}
