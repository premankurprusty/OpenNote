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
    None,
}
