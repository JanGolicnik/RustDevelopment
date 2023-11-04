use super::token::Token;
use super::CompilationError;

pub struct Tokens {
    tokens: Vec<Token>,
    index: usize,
    line_num: usize,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Tokens { tokens, index: 0, line_num: 1 }
    }

    pub fn next(&mut self) -> Result<&Token, CompilationError> {
        self.index += 1;

        loop{
            match self.tokens.get(self.index - 1) {
                Some(t) => match t {
                    Token::EndLine => self.line_num += 1,
                    _=> return Ok(t),
                }
                None=> return Err(CompilationError::new("Missing Token")),
            }
            self.index += 1;
        }
    }

    pub fn peek(&mut self, mut offset: usize) -> Result<&Token, CompilationError> {
        loop{
            match self.tokens.get(self.index + offset) {
                Some(t) => match t {
                    Token::EndLine => {},
                    _=> return Ok(t),
                }
                None=> return Err(CompilationError::new("Missing Token")),
            }
            offset += 1;
        }
    }

    pub fn get_line_num(&self) -> usize {
        self.line_num
    }
}