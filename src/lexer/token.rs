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
}
