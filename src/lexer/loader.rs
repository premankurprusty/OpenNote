use crate::error::LexerError;
use std::fs;
use std::path::Path;

pub(super) fn load(filename: &str) -> Result<String, LexerError> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(LexerError::FileNotFound {
            path: filename.to_string(),
        });
    };

    if path.extension().and_then(|extension| extension.to_str()) != Some("txt") {
        return Err(LexerError::InvalidExtension {
            error: "File must be a .txt file".to_string(),
        });
    };

    let contents = fs::read_to_string(&path).unwrap();
    Ok(contents)
}
