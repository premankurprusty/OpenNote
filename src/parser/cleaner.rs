use crate::parser::tempinlined::{TempInline, TempInlined};

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
    merge_blocks(cleaned_blocks)
}

fn clean_inline(contents: &mut Vec<TempInline>) -> Vec<TempInline> {
    let mut cleaned_contents = Vec::new();
    for item in contents {
        match item {
            TempInline::Text(text) => cleaned_contents.push(TempInline::Text(text.clone())),
            TempInline::Bold(contents) => {
                if contents.contains(&TempInline::ToBeCleaned("<".to_string())) {
                    cleaned_contents.append(&mut clean_inline(contents));
                } else {
                    cleaned_contents.push(TempInline::Bold(clean_inline(contents)));
                }
            }
            TempInline::Italic(contents) => {
                if contents.contains(&TempInline::ToBeCleaned("[".to_string())) {
                    cleaned_contents.append(&mut clean_inline(contents));
                } else {
                    cleaned_contents.push(TempInline::Italic(clean_inline(contents)));
                }
            }
            TempInline::NewLine => {
                cleaned_contents.push(TempInline::NewLine);
            }
            TempInline::ToBeCleaned(content) => {
                cleaned_contents.push(TempInline::Text(content.clone()))
            }
        }
    }
    merge(cleaned_contents)
}

fn merge(contents: Vec<TempInline>) -> Vec<TempInline> {
    let mut cleaned_contents = Vec::new();
    let mut buffer = String::new();
    for item in contents {
        if let TempInline::Text(content) = item {
            buffer.push_str(&content);
        } else {
            if !buffer.is_empty() {
                cleaned_contents.push(TempInline::Text(buffer.clone()));
                buffer.clear();
            }
            cleaned_contents.push(item);
        }
    }
    if !buffer.is_empty() {
        cleaned_contents.push(TempInline::Text(buffer));
    }

    cleaned_contents
}

fn merge_blocks(blocks: Vec<TempInlined>) -> Vec<TempInlined> {
    let mut tempdoc = Vec::new();

    for block in blocks {
        match block {
            TempInlined::Header(level, content) => {
                tempdoc.push(TempInlined::Header(level, content));
            }
            TempInlined::Paragraph(content) => {
                tempdoc.push(TempInlined::Paragraph(content));
            }
            TempInlined::Notype(mut content) => {
                if let Some(last) = tempdoc.last_mut() {
                    last.content_mut().append(&mut content);
                } else {
                    tempdoc.push(TempInlined::Notype(content));
                }
            }
        }
    }

    tempdoc
}
