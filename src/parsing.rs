use std::collections::HashMap;

use crate::compilation_error::CompilationError;
use crate::parsing_nodes::ProgramNode;
use crate::tokenization::Tokens;

pub struct Variable {
    pub stack_position: usize,
}

pub struct ParsingContext {
    stack_size: usize,
    pub variables: HashMap<String, Variable>,
    pub output: String,
}

impl ParsingContext {
    pub fn push_on_stack(&mut self, register: &str) {
        self.push_line(format!("    push {}", register).as_str());
        self.stack_size += 8;
    }

    pub fn pop_from_stack(&mut self, register: &str) {
        self.push_line(format!("    pop {}", register).as_str());
        self.stack_size -= 8;
    }

    pub fn push_line(&mut self, string: &str) {
        self.output.push_str(string);
        self.output.push('\n');
    }

    pub fn stack_size(&self) -> usize {
        self.stack_size
    }
}

pub fn parse(tokens: &mut Tokens) -> Result<String, CompilationError> {
    let mut parsing_context: ParsingContext = ParsingContext {
        stack_size: 0,
        variables: HashMap::new(),
        output: String::new(),
    };

    let root = ProgramNode::parse(tokens)?;

    root.to_asm(&mut parsing_context)?;

    Ok(parsing_context.output)
}
