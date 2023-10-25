use crate::{
    compilation_error::CompilationError,
    parsing::ParsingContext,
    tokenization::{OperatorInfo, Token, Tokens},
};

pub struct ProgramNode {
    statements: Vec<StatementNode>,
}

enum StatementNode {
    Declaration {
        literal: String,
        expr: Expression,
    },
    Return {
        expr: Expression,
    },
    StartScope,
    EndScope,
    If {
        expr: Expression,
        statements: Vec<StatementNode>,
    },
}

enum Term {
    Int(i32),
    Identifier(String),
}

enum Expression {
    Term(Term),
    Binary {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Term {
    fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let token = tokens.next()?;
        match token {
            Token::Int(int_token_val) => Ok(Term::Int(*int_token_val)),
            Token::Identifier(name) => {
                let name = name.clone();
                Ok(Term::Identifier(name))
            }
            _ => Err(CompilationError::new(
                format!("Weird term {:?}", token).as_str(),
            )),
        }
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            Term::Int(val) => {
                parsing_context.push_line(format!("    mov rdi, {}", val).as_str());
                Ok(())
            }
            Term::Identifier(name) => {
                if let Some(var) = parsing_context.get_var(name) {
                    parsing_context
                        .push_line(format!("    mov rdi, [rbp - {}]", var.stack_position).as_str());
                    Ok(())
                } else {
                    Err(CompilationError::new(
                        format!("Undeclared variable {name}").as_str(),
                    ))
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

                if !matches!(
                    token,
                    Token::Plus
                        | Token::Star
                        | Token::Slash
                        | Token::Minus
                        | Token::LessThan
                        | Token::GreaterThan
                        | Token::Equals
                ) {
                    return Err(CompilationError::new("invalid expression operator"));
                }

                println!("CREATED EXPRESSION WITH TOKEN {:?}", token);

                expr = Expression::Binary {
                    operator: token,
                    left: Box::new(expr),
                    right: Box::new(right_expression),
                };
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
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                left.to_asm(parsing_context)?;
                parsing_context.push_on_stack("rdi");
                right.to_asm(parsing_context)?;
                parsing_context.pop_from_stack("rax");

                match operator {
                    Token::Plus => {
                        parsing_context.push_line("    add rax, rdi ");
                        parsing_context.push_line("    mov rdi, rax");
                    }
                    Token::Minus => {
                        parsing_context.push_line("    sub rax, rdi");
                        parsing_context.push_line("    mov rdi, rax");
                    }
                    Token::Slash => {
                        parsing_context.push_line("    div rdi");
                        parsing_context.push_line("    mov rdi, rax");
                    }
                    Token::Star => {
                        parsing_context.push_line("    mul rdi");
                        parsing_context.push_line("    mov rdi, rax");
                    }
                    Token::LessThan => {
                        let true_label = parsing_context.new_label();
                        let false_label = parsing_context.new_label();
                        let label = parsing_context.new_label();
                        parsing_context.push_line("    cmp rax, rdi");
                        parsing_context.push_line(format!("    jl {true_label}").as_str());
                        parsing_context.push_line(format!("{false_label}:").as_str());
                        parsing_context.push_line("    mov rdi, 0");

                        parsing_context.push_line(format!("    jmp {label}").as_str());
                        parsing_context.push_line(format!("{true_label}:").as_str());
                        parsing_context.push_line("    mov rdi, 1");

                        parsing_context.push_line(format!("{label}:").as_str());
                    }
                    Token::GreaterThan => {
                        let true_label = parsing_context.new_label();
                        let false_label = parsing_context.new_label();
                        let label = parsing_context.new_label();
                        parsing_context.push_line("    cmp rax, rdi");
                        parsing_context.push_line(format!("    jg {true_label}").as_str());
                        parsing_context.push_line(format!("{false_label}:").as_str());
                        parsing_context.push_line("    mov rdi, 0");
                        parsing_context.push_line(format!("    jmp {label}").as_str());
                        parsing_context.push_line(format!("{true_label}:").as_str());
                        parsing_context.push_line("    mov rdi, 1");
                        parsing_context.push_line(format!("{label}:").as_str());
                    }
                    Token::Equals => {
                        let true_label = parsing_context.new_label();
                        let false_label = parsing_context.new_label();
                        let label = parsing_context.new_label();
                        parsing_context.push_line("    cmp rax, rdi");
                        parsing_context.push_line(format!("    je {true_label}").as_str());
                        parsing_context.push_line(format!("{false_label}:").as_str());
                        parsing_context.push_line("    mov rdi, 0");
                        parsing_context.push_line(format!("    jmp {label}").as_str());
                        parsing_context.push_line(format!("{true_label}:").as_str());
                        parsing_context.push_line("    mov rdi, 1");
                        parsing_context.push_line(format!("{label}:").as_str());
                    }
                    _ => {
                        return Err(CompilationError::new(
                            "invalid binary operator (how did you get here?)",
                        ));
                    }
                }
                Ok(())
            }
        }
    }
}

impl StatementNode {
    fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        Ok(match tokens.next()? {
            Token::Return => StatementNode::parse_return(tokens)?,
            Token::Declaration => StatementNode::parse_declaration(tokens)?,
            Token::If => StatementNode::parse_if(tokens)?,
            Token::OpenCurly => StatementNode::StartScope,
            Token::ClosedCurly => StatementNode::EndScope,
            _ => {
                return Err(CompilationError::new("Unexpected token"));
            }
        })
    }

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

    fn parse_if(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let expr = Expression::parse(tokens, 0)?;
        match tokens.next()? {
            Token::OpenCurly => {
                let mut statements: Vec<StatementNode> = Vec::new();
                let mut inner_start_scopes = 0;

                loop {
                    let stmt = match tokens.next()? {
                        Token::Return => StatementNode::parse_return(tokens)?,
                        Token::Declaration => StatementNode::parse_declaration(tokens)?,
                        Token::If => StatementNode::parse_if(tokens)?,
                        Token::OpenCurly => {
                            inner_start_scopes += 1;
                            StatementNode::StartScope
                        }
                        Token::ClosedCurly => {
                            inner_start_scopes -= 1;
                            if inner_start_scopes <= 0 {
                                break;
                            }
                            StatementNode::EndScope
                        }
                        _ => {
                            return Err(CompilationError::new("Unexpected token"));
                        }
                    };

                    statements.push(stmt);
                }

                Ok(StatementNode::If { expr, statements })
            }
            _ => Err(CompilationError::new("expected {")),
        }
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            StatementNode::Declaration { literal, expr } => {
                expr.to_asm(parsing_context)?;
                let name = literal.clone();
                parsing_context.push_on_stack("rdi");
                if parsing_context.add_var(name).is_none() {
                    return Err(CompilationError::new(
                        format!("Variable {literal} already exists").as_str(),
                    ));
                }
            }
            StatementNode::Return { expr } => {
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rax, 60");
                parsing_context.push_line("    syscall");
            }
            StatementNode::StartScope => {
                parsing_context.push_scope();
            }
            StatementNode::EndScope => {
                parsing_context.pop_scope();
            }
            StatementNode::If { expr, statements } => {
                let label = parsing_context.new_label();
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    cmp rdi, 0");
                parsing_context.push_line(format!("    je {label}").as_str());
                for stmt in statements {
                    stmt.to_asm(parsing_context)?;
                }
                parsing_context.push_line(format!("{label}:").as_str());
            }
            _ => {
                return Err(CompilationError::new("statement not implemented yet"));
            }
        }
        Ok(())
    }
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
