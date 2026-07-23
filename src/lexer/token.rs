#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BoldInit,
    BoldEnd,
    ItalicInit,
    ItalicEnd,
    HeaderInit,
    SubheaderInit,
    ContentInit,
    NoInit,
    Text(String),
    Newline,
    //None,
}

//impl Token {
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
//}
