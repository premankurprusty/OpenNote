use crate::lexer::token::Token;
#[derive(Debug)]
pub enum TempBlock {
    Header(HeaderBlock),
    Content(ContentBlock),
    None(NoBlock),
}

impl TempBlock {
    pub fn push(&mut self, token: Token) {
        match self {
            TempBlock::Header(header) => header.push(token),
            TempBlock::Content(content) => content.push(token),
            TempBlock::None(no_block) => no_block.push(token),
        }
    }
    pub fn contents(&self) -> Vec<Token> {
        match self {
            TempBlock::Header(header) => header.children.clone(),
            TempBlock::Content(content) => content.children.clone(),
            TempBlock::None(no_block) => no_block.children.clone(),
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
