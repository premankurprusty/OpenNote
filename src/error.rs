#[derive(Debug)]
pub enum LexerError {
    FileNotFound { path: String },
    InvalidExtension { error: String },
}
