use std::fmt;

#[derive(Debug)]
pub struct TokenizationError {
    message: String,
}

impl TokenizationError {
    pub fn new(message: &str) -> TokenizationError {
        TokenizationError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenization Error: {}", self.message)
    }
}

impl std::error::Error for TokenizationError {}
