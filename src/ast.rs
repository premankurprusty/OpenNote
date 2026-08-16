#[derive(Debug)]
pub struct Document {
    pub nodes: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    NewLine,
}

impl IntoIterator for Document {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Block>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}
