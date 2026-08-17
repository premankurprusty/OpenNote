use crate::ast::{Block, Document, Inline};

pub(super) fn html_build(document: Document) -> String {
    let mut result = String::new();
    for block in document.nodes {
        match block {
            Block::Header {
                level,
                text,
                children,
            } => {
                result.push_str(&format!("<h{}>\n", level + 1));
                result.push_str(&html_text(text));
                result.push_str(&format!("</h{}>\n", level + 1));
                result.push_str(&html_build(Document { nodes: children }));
            }
            Block::Paragraph { text } => {
                result.push_str("<p>\n");
                result.push_str(&html_text(text));
                result.push_str("</p>\n");
            }
        }
    }
    result
}

fn html_text(text: Vec<Inline>) -> String {
    let mut result = String::new();
    for item in text {
        match item {
            Inline::Text(content) => {
                result.push_str(
                    &content
                        .replace("&", "&amp")
                        .replace("<", "&lt")
                        .replace(">", "&gt"),
                );
            }
            Inline::Bold(content) => {
                result.push_str("<strong>");
                result.push_str(&html_text(content));
                result.push_str("</strong>");
            }
            Inline::Italic(content) => {
                result.push_str("<em>");
                result.push_str(&html_text(content));
                result.push_str("</em>");
            }
            Inline::NewLine => {
                result.push_str("<br>\n");
            }
        }
    }
    result
}
