#[derive(Debug)]
pub enum LexerError {
    FileNotFound { path: String },
    InvalidExtension { error: String },
    IoError { path: String, error: std::io::Error },
}
