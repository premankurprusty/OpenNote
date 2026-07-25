pub struct Document {
    pub nodes: Vec<Block>,
}

pub enum Block {
    Header {
        level: usize,
        text: Vec<Inline>,
        children: Vec<Block>,
    },
    Paragraph {
        text: Vec<Inline>,
    },
}

pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    NewLine,
}
