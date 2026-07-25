use crate::ast::Inline;
use crate::parser::tempinlined::TempInlined;

pub(super) fn clean(blocks: Vec<TempInlined>) -> Vec<TempInlined> {
    let mut cleaned_blocks = Vec::new();
    for block in blocks {
        let mut contents = block.clone().contents();
        let cleaned_contents = clean_inline(&mut contents);
        match block {
            TempInlined::Header(level, _) => {
                cleaned_blocks.push(TempInlined::Header(level, cleaned_contents))
            }
            TempInlined::Paragraph(_) => {
                cleaned_blocks.push(TempInlined::Paragraph(cleaned_contents))
            }
            TempInlined::Notype(_) => cleaned_blocks.push(TempInlined::Notype(cleaned_contents)),
        }
    }
    cleaned_blocks
}

fn clean_inline(contents: &mut Vec<Inline>) -> Vec<Inline> {
    let mut cleaned_contents = Vec::new();
    for item in contents {
        match item {
            Inline::Text(text) => cleaned_contents.push(Inline::Text(text.clone())),
            Inline::Bold(contents) => {
                if contents.contains(&Inline::ToBeCleaned("<".to_string())) {
                    cleaned_contents.append(&mut clean_inline(contents));
                } else {
                    cleaned_contents.push(Inline::Bold(clean_inline(contents)));
                }
            }
            Inline::Italic(contents) => {
                if contents.contains(&Inline::ToBeCleaned("[".to_string())) {
                    cleaned_contents.append(&mut clean_inline(contents));
                } else {
                    cleaned_contents.push(Inline::Italic(clean_inline(contents)));
                }
            }
            Inline::NewLine => {
                cleaned_contents.push(Inline::NewLine);
            }
            Inline::ToBeCleaned(content) => cleaned_contents.push(Inline::Text(content.clone())),
        }
    }
    cleaned_contents
}

//fn merge(contents: &mut Vec<Inline>) -> Vec<Inline> {}
