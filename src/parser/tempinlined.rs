#[derive(Debug, Clone)]
pub enum TempInlined {
    Header(usize, Vec<TempInline>),
    Paragraph(Vec<TempInline>),
    Notype(Vec<TempInline>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TempInline {
    Text(String),
    Bold(Vec<TempInline>),
    Italic(Vec<TempInline>),
    NewLine,
    ToBeCleaned(String),
}

impl TempInlined {
    pub fn contents(self) -> Vec<TempInline> {
        match self {
            TempInlined::Header(_, content) => content.clone(),
            TempInlined::Paragraph(content) => content.clone(),
            TempInlined::Notype(content) => content.clone(),
        }
    }

    pub fn content_mut(&mut self) -> &mut Vec<TempInline> {
        match self {
            TempInlined::Header(_, c) | TempInlined::Paragraph(c) | TempInlined::Notype(c) => c,
        }
    }
}
