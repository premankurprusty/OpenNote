use crate::ast::Inline;
#[derive(Debug, Clone)]
pub enum TempInlined {
    Header(usize, Vec<Inline>),
    Paragraph(Vec<Inline>),
    Notype(Vec<Inline>),
}

impl TempInlined {
    pub fn contents(self) -> Vec<Inline> {
        match self {
            TempInlined::Header(_, content) => content.clone(),
            TempInlined::Paragraph(content) => content.clone(),
            TempInlined::Notype(content) => content.clone(),
        }
    }
}
