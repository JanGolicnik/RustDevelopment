use std::fmt;

#[derive(Debug)]
pub struct CompilationError {
    message: String,
}

impl CompilationError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Compilation Error: {}", self.message)
    }
}

impl std::error::Error for CompilationError {}
