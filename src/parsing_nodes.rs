use crate::{
    compilation_error::CompilationError,
    parsing::{ParsingContext, Variable},
    tokenization::{Token, Tokens},
};

pub struct ProgramNode {
    statements: Vec<StatementNode>,
}

enum StatementNode {
    Declaration { literal: String, expr: Expression },
    Return { expr: Expression },
}

enum Term {
    Int(i32),
    Identifier(String),
}

enum Expression {
    Term(Term),
    Addition {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // Multiplication {
    //     left: Box<Expression>,
    //     right: Box<Expression>,
    // },
}

impl Term {
    fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        match tokens.next()? {
            Token::Int(int_token_val) => Ok(Term::Int(*int_token_val)),
            Token::Identifier(name) => {
                let name = name.clone();
                Ok(Term::Identifier(name))
            }
            _ => Err(CompilationError::new("Weird term")),
        }
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            Term::Int(val) => {
                parsing_context.push_line(format!("    mov rdi, {}", val).as_str());
                parsing_context.push_on_stack("rdi");
                Ok(())
            }
            Term::Identifier(name) => {
                if let Some(var) = parsing_context.variables.get(name) {
                    parsing_context
                        .push_line(format!("    mov rdi, [rbp - {}]", var.stack_position).as_str());
                    parsing_context.push_on_stack("rdi");
                    Ok(())
                } else {
                    Err(CompilationError::new("Undeclared variable {name}"))
                }
            }
            _ => Err(CompilationError::new("term not implemented yet")),
        }
    }
}

impl Expression {
    fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let left_term = Term::parse(tokens)?;
        let term_expression = Expression::Term(left_term);
        match tokens.peek(0)? {
            Token::Addition => {
                tokens.next()?;
                let right = Expression::parse(tokens)?;
                Ok(Expression::Addition {
                    left: Box::new(term_expression),
                    right: Box::new(right),
                })
            }
            Token::EndStatement => Ok(term_expression),
            _ => Err(CompilationError::new(
                format!("unexpected token {:?}", tokens.peek(1)).as_str(),
            )),
        }
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            Expression::Term(term) => {
                term.to_asm(parsing_context)?;
                Ok(())
            }
            Expression::Addition { left, right } => {
                left.to_asm(parsing_context)?;
                right.to_asm(parsing_context)?;
                parsing_context.pop_from_stack("rax");
                parsing_context.pop_from_stack("rdi");
                parsing_context.push_line("    add rdi, rax");
                parsing_context.push_on_stack("rdi");
                Ok(())
            }
            _ => Err(CompilationError::new("not implemented yet")),
        }
    }
}

impl StatementNode {
    fn parse_return(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let expr = Expression::parse(tokens)?;
        match tokens.next()? {
            Token::EndStatement => Ok(StatementNode::Return { expr }),
            _ => Err(CompilationError::new("expected semicolon")),
        }
    }

    fn parse_declaration(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        match tokens.next()? {
            Token::Identifier(name) => {
                let name = name.clone();

                match tokens.next()? {
                    Token::Equals => {
                        let expr = Expression::parse(tokens)?;
                        match tokens.next()? {
                            Token::EndStatement => Ok(StatementNode::Declaration {
                                literal: name,
                                expr,
                            }),
                            _ => Err(CompilationError::new("expected end statement")),
                        }
                    }
                    _ => Err(CompilationError::new("expected equals ")),
                }
            }
            _ => Err(CompilationError::new("expected identifier")),
        }
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            StatementNode::Declaration { literal, expr } => {
                expr.to_asm(parsing_context)?;
                let name = literal.clone();
                if parsing_context
                    .variables
                    .insert(
                        name,
                        Variable {
                            stack_position: parsing_context.stack_size(),
                        },
                    )
                    .is_some()
                {
                    Err(CompilationError::new("Variable {name} already exists"))
                } else {
                    Ok(())
                }
            }
            StatementNode::Return { expr } => {
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rax, 60");
                parsing_context.pop_from_stack("rdi");
                parsing_context.push_line("    syscall");
                Ok(())
            }
        }
    }
}

impl ProgramNode {
    pub fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let mut statements: Vec<StatementNode> = Vec::new();

        while let Ok(token) = tokens.next() {
            let stmt = match token {
                Token::Return => StatementNode::parse_return(tokens)?,
                Token::Declaration => StatementNode::parse_declaration(tokens)?,
                _ => {
                    return Err(CompilationError::new("Unexpected token"));
                }
            };

            statements.push(stmt)
        }

        Ok(Self { statements })
    }

    pub fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        parsing_context.push_line("global _start:");
        parsing_context.push_line("_start:");
        parsing_context.push_line("    push rbp");
        parsing_context.push_line("    mov rbp, rsp");

        for stmt in &self.statements {
            stmt.to_asm(parsing_context)?;
        }
        Ok(())
    }
}
