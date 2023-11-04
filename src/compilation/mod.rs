use compilation_error::CompilationError;

use self::{tokenization::tokenize, parsing::parse};

mod parsing;
mod tokenization;
mod compilation_error;

pub fn compile_to_asm(code: &String) -> Result<String, CompilationError> {
    match tokenize(&code) {
        Ok(mut tokens) => parse(&mut tokens),
        Err(e) => Err(e),
    }
}