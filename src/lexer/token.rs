#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BoldInit,
    BoldEnd,
    ItalicInit,
    ItalicEnd,
    HeaderInit(usize),
    ContentInit,
    NoInit,
    Text(String),
    Newline,
    //None,
}

pub enum TokenType {
    Block,
    Inline,
}

impl Token {
    //    pub fn to_string(&self) -> String {
    //        match self {
    //            Token::BoldInit => "BoldInit".to_string(),
    //            Token::BoldEnd => "BoldEnd".to_string(),
    //            Token::ItalicInit => "ItalicInit".to_string(),
    //            Token::ItalicEnd => "ItalicEnd".to_string(),
    //            Token::HeaderInit => "HeaderInit".to_string(),
    //            Token::SubheaderInit => "SubheaderInit".to_string(),
    //            Token::ContentInit => "ContentInit".to_string(),
    //            Token::NoInit => "NoInit".to_string(),
    //            Token::Text(text) => text.clone(),
    //            Token::Newline => "Newline".to_string(),
    //            Token::None => "None".to_string(),
    //        }
    //    }

    pub fn type_of(&self) -> TokenType {
        match self {
            Token::BoldInit => TokenType::Inline,
            Token::BoldEnd => TokenType::Inline,
            Token::ItalicInit => TokenType::Inline,
            Token::ItalicEnd => TokenType::Inline,
            Token::HeaderInit(_) => TokenType::Block,
            Token::ContentInit => TokenType::Block,
            Token::NoInit => TokenType::Block,
            Token::Text(_) => TokenType::Inline,
            Token::Newline => TokenType::Inline,
            //Token::None => "None",
        }
    }
}
