use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilationError {
    message: String,
    line_num: Option<usize>
}

impl CompilationError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            line_num: None
        }
    }

    pub fn add_line_num(&mut self, line_num: usize) -> &mut Self {
        self.line_num = Some(line_num);
        self
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.line_num {
            Some(line_num) => write!(f, "Compilation Error: {} on line {}", self.message, line_num),
            None => write!(f, "Compilation Error: {}", self.message)
        }
    }
}

impl std::error::Error for CompilationError {}
