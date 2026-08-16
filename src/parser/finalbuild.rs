use crate::{
    ast::{Block, Document, Inline},
    parser::tempinlined::{TempInline, TempInlined},
};

pub(super) fn build_final(blocks: Vec<TempInlined>) -> Document {
    let mut nodes = Vec::new();
    for block in blocks {
        match block {
            TempInlined::Header(level, text) => {
                nodes.push(Block::Header {
                    level,
                    text: final_inline(text),
                    children: Vec::new(),
                });
            }
            TempInlined::Paragraph(text) => {
                nodes.push(Block::Paragraph {
                    text: final_inline(text),
                });
            }
            TempInlined::Notype(text) => {
                nodes.push(Block::Paragraph {
                    text: final_inline(text),
                });
            }
        }
    }
    nodes = build_tree(nodes);
    Document { nodes }
}

fn final_inline(text: Vec<TempInline>) -> Vec<Inline> {
    let mut inlines = Vec::new();
    for inline in text {
        match inline {
            TempInline::Text(text) => {
                inlines.push(Inline::Text(text));
            }
            TempInline::Bold(text) => {
                inlines.push(Inline::Bold(final_inline(text)));
            }
            TempInline::Italic(text) => {
                inlines.push(Inline::Italic(final_inline(text)));
            }
            TempInline::NewLine => {
                inlines.push(Inline::NewLine);
            }
            TempInline::ToBeCleaned(text) => {
                inlines.push(Inline::Text(text));
            }
        }
    }
    inlines
}

fn build_tree(blocks: Vec<Block>) -> Vec<Block> {
    fn build_level(
        blocks: &mut std::iter::Peekable<std::vec::IntoIter<Block>>,
        parent_level: Option<usize>,
    ) -> Vec<Block> {
        let mut result = Vec::new();
        while let Some(block) = blocks.peek() {
            if let Block::Header { level, .. } = block {
                if parent_level.is_some_and(|parent| *level <= parent) {
                    break;
                }
            }
            let block = blocks.next().unwrap();

            match block {
                Block::Header { level, text, .. } => {
                    let children = build_level(blocks, Some(level));
                    result.push(Block::Header {
                        level,
                        text,
                        children,
                    })
                }
                Block::Paragraph { text } => result.push(Block::Paragraph { text }),
            }
        }
        result
    }
    build_level(&mut blocks.into_iter().peekable(), None)
}
