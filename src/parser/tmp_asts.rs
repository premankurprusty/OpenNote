use crate::lexer::token::Token;
#[derive(Debug)]
pub enum Block {
    Header(HeaderBlock),
    Content(ContentBlock),
    None(NoBlock),
}

impl Block {
    pub fn push(&mut self, token: Token) {
        match self {
            Block::Header(header) => header.push(token),
            Block::Content(content) => content.push(token),
            Block::None(no_block) => no_block.push(token),
        }
    }
}

#[derive(Debug)]
pub struct NoBlock {
    pub children: Vec<Token>,
}

#[derive(Debug)]
pub struct HeaderBlock {
    pub level: usize,
    pub children: Vec<Token>,
}

#[derive(Debug)]
pub struct ContentBlock {
    pub children: Vec<Token>,
}

impl HeaderBlock {
    pub fn new(level: usize) -> Self {
        Self {
            level,
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.children.push(token);
    }
}

impl Default for HeaderBlock {
    fn default() -> Self {
        Self::new(0)
    }
}

impl ContentBlock {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.children.push(token);
    }
}

impl Default for ContentBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl NoBlock {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.children.push(token);
    }
}

impl Default for NoBlock {
    fn default() -> Self {
        Self::new()
    }
}
