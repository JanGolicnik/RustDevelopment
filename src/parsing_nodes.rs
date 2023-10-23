use crate::{
    compilation_error::CompilationError,
    parsing::{ParsingContext, Variable},
    tokenization::{OperatorInfo, Token, Tokens},
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
    Multiplication {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Subtraction {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Division {
        left: Box<Expression>,
        right: Box<Expression>,
    },
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
        }
    }
}

impl Expression {
    fn parse(tokens: &mut Tokens, min_precedence: usize) -> Result<Self, CompilationError> {
        let mut expr = match tokens.peek(0)? {
            Token::OpenBracket => {
                tokens.next()?;
                let val = Expression::parse(tokens, 0)?;
                match tokens.next()? {
                    Token::ClosedBracket => val,
                    _ => {
                        return Err(CompilationError::new("Expected close paranthases"));
                    }
                }
            }
            _ => Expression::Term(Term::parse(tokens)?),
        };

        loop {
            let token = tokens.peek(0)?.clone();
            if let Some(OperatorInfo(precedence, associative)) = token.get_operator_info() {
                if precedence < min_precedence {
                    break;
                }

                let next_min_precedence = if associative {
                    precedence + 1
                } else {
                    precedence
                };
                tokens.next()?;
                let right_expression = Expression::parse(tokens, next_min_precedence)?;

                match token {
                    Token::Plus => {
                        expr = Expression::Addition {
                            left: Box::new(expr),
                            right: Box::new(right_expression),
                        }
                    }
                    Token::Star => {
                        expr = Expression::Multiplication {
                            left: Box::new(expr),
                            right: Box::new(right_expression),
                        }
                    }
                    Token::Slash => {
                        expr = Expression::Division {
                            left: Box::new(expr),
                            right: Box::new(right_expression),
                        }
                    }
                    Token::Minus => {
                        expr = Expression::Subtraction {
                            left: Box::new(expr),
                            right: Box::new(right_expression),
                        }
                    }
                    _ => return Err(CompilationError::new("invalid operator")),
                }
            } else {
                break;
            }
        }

        Ok(expr)
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
                parsing_context.pop_from_stack("rdi");
                parsing_context.pop_from_stack("rax");
                parsing_context.push_line("    add rdi, rax");
                parsing_context.push_on_stack("rdi");
                Ok(())
            }
            Expression::Multiplication { left, right } => {
                left.to_asm(parsing_context)?;
                right.to_asm(parsing_context)?;
                parsing_context.pop_from_stack("rdi");
                parsing_context.pop_from_stack("rax");
                parsing_context.push_line("    mul rdi");
                parsing_context.push_on_stack("rax");
                Ok(())
            }
            Expression::Subtraction { left, right } => {
                left.to_asm(parsing_context)?;
                right.to_asm(parsing_context)?;
                parsing_context.pop_from_stack("rdi");
                parsing_context.pop_from_stack("rax");
                parsing_context.push_line("    sub rax, rdi");
                parsing_context.push_on_stack("rax");
                Ok(())
            }
            Expression::Division { left, right } => {
                left.to_asm(parsing_context)?;
                right.to_asm(parsing_context)?;
                parsing_context.pop_from_stack("rdi");
                parsing_context.pop_from_stack("rax");
                parsing_context.push_line("    div rdi");
                parsing_context.push_on_stack("rax");
                Ok(())
            }
        }
    }
}

impl StatementNode {
    fn parse_return(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let expr = Expression::parse(tokens, 0)?;
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
                        let expr = Expression::parse(tokens, 0)?;
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
