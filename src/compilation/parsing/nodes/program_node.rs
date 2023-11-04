use super::super::super::tokenization::Tokens;
use super::super::CompilationError;
use super::super::ParsingContext;
use super::statement_node::StatementNode;

pub struct ProgramNode {
    statements: Vec<StatementNode>,
}

impl ProgramNode {
    pub fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let mut statements: Vec<StatementNode> = Vec::new();

        while tokens.peek(1).is_ok() {
            let stmt = StatementNode::parse(tokens)?;
            statements.push(stmt);
        }

        Ok(Self { statements })
    }

    pub fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        for stmt in &self.statements {
            if let StatementNode::Function {
                name: _,
                scope: _,
                args: _,
            } = stmt
            {
                stmt.to_asm(parsing_context)?;
            }
        }

        parsing_context.push_line("section .text");
        parsing_context.push_line("global _start:");
        parsing_context.push_line("_start:");
        parsing_context.push_line("    push rbp");
        parsing_context.push_line("    mov rbp, rsp");

        for stmt in &self.statements {
            match stmt {
                StatementNode::Function {
                    name: _,
                    scope: _,
                    args: _,
                } => {}
                _ => stmt.to_asm(parsing_context)?,
            }
        }

        parsing_context.push_line("section .data");
        let lines: Vec<String> = parsing_context
            .strings
            .iter()
            .map(|global_str| {
                let label = global_str.label.clone();
                let string = global_str.string.clone();
                format!("{}:\n    db \"{}\", 10", label, string)
            })
            .collect();

        for line in lines {
            parsing_context.push_line(line.as_str());
        }

        Ok(())
    }
}
