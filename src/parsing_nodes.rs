use crate::{
    compilation_error::CompilationError,
    match_token,
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
        size: usize,
    },
    Return {
        expr: Expression,
    },
    Scope {
        statements: Vec<StatementNode>,
    },
    If {
        expr: Expression,
        scope: Box<StatementNode>,
    },
    While {
        expr: Expression,
        scope: Box<StatementNode>,
    },
    Assignment {
        literal: String,
        expr: Expression,
        index_expr: Option<Expression>,
    },
    Break,
    Print {
        expr: Expression,
    },
    Function {
        name: String,
        scope: Box<StatementNode>,
        args: Vec<String>,
    },
}

#[derive(Debug)]
enum Term {
    Int(i32),
    Identifier(String, Option<Box<Expression>>),
    String(String),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug)]
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
        match_token!(tokens.next()?, "weird term",
            Token::Int(int_token_val) => {Ok(Term::Int(*int_token_val))},
            Token::String(val) => {Ok(Term::String(val.clone()))},
            Token::Identifier(name) => {
                    let name = name.clone();
                    match_token!(tokens.peek(0)?,
                         Token::OpenBracket => {
                            tokens.next()?;
                            let mut expressions: Vec<Expression> = Vec::new();

                            while match_token!(tokens.peek(0)?,
                                Token::ClosedBracket => {
                                    tokens.next()?;
                                    false
                                },
                                Token::Comma => {
                                    tokens.next()?;
                                    true
                                },
                                _=> {
                                    let expr = Expression::parse(tokens, 0)?;
                                    expressions.push(expr);
                                    true
                                }
                            ) {}

                            Ok(Term::FunctionCall(name, expressions))
                        },
                        Token::OpenSquare => {
                            tokens.next()?;
                            let expr = Expression::parse(tokens, 0)?;
                            match_token!(tokens.next()?, 
                                Token::ClosedSquare => {
                                    Ok(Term::Identifier(name, Some(Box::new(expr))))
                                }, "expected ]")
                        },
                        _ => {
                            Ok(Term::Identifier(name, None))
                        }
                    )
                }
        )
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            Term::Int(val) => {
                parsing_context.push_line(format!("    mov rdi, {}", val).as_str());
                Ok(())
            }
            Term::Identifier(name, index_expr) => {
                if let Some(var) = parsing_context.get_var(name) {
                    let sign = if var.stack_position < 0 { "+" } else { "-" };
                    match index_expr {
                        Some(index_expr) => {
                            index_expr.to_asm(parsing_context)?;
                            parsing_context.push_line("    mov rax, rdi");

                            parsing_context.push_line(
                                format!("    mov rdi, [rbp {} {} + rax * 4]", sign, var.stack_position.abs())
                                    .as_str(),
                            );
                        }
                        None => {
                            parsing_context.push_line(
                                format!("    mov rdi, [rbp {} {}]", sign, var.stack_position.abs())
                                    .as_str(),
                            );
                        }
                    }
                    Ok(())
                } else {
                    Err(CompilationError::new(
                        format!("Undeclared variable {name}").as_str(),
                    ))
                }
            }
            Term::String(val) => {
                let label = parsing_context.add_string(val);
                parsing_context.push_line(format!("    mov rdi, {}", label).as_str());
                Ok(())
            }
            Term::FunctionCall(name, args) => {
                if parsing_context.function_exists(name) {
                    for i in 0..args.len() {
                        if let Some(arg) = args.get(i) {
                            arg.to_asm(parsing_context)?;
                            parsing_context.push_on_stack("rdi");
                        }
                    }
                    parsing_context.push_line(format!("    call {}", name).as_str());
                    Ok(())
                } else {
                    Err(CompilationError::new("undefined function"))
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
                match_token!(tokens.next()?, Token::ClosedBracket => {val}, "expected )")
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

                match_token!(
                    &token,
                    Token::Plus
                        | Token::Star
                        | Token::Slash
                        | Token::Minus
                        | Token::LessThan
                        | Token::GreaterThan
                        | Token::Equals,
                    "invalid expression operator"
                );

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
        let node = match_token!(tokens.next()?, "unexpected token",
            Token::Return => {StatementNode::parse_return(tokens)?},
            Token::Declaration => {StatementNode::parse_declaration(tokens)?},
            Token::If => {StatementNode::parse_if(tokens)?},
            Token::OpenCurly => {StatementNode::parse_scope(tokens)?},
            Token::While => {StatementNode::parse_while(tokens)?},
            Token::Break => {match tokens.next()? {
                Token::EndStatement => StatementNode::Break,
                _ => {
                    return Err(CompilationError::new("Expected ;"));
                }
            }},
            Token::Print => {
                let expr = Expression::parse(tokens, 0)?;
                match tokens.next()? {
                    Token::EndStatement => StatementNode::Print { expr },
                    _ => {
                        return Err(CompilationError::new("Expected ;"));
                    }
                }
            },
            Token::Identifier(name) => {
                let var_name = name.clone();
                StatementNode::parse_assignment(tokens, var_name)?
            },
            Token::Function => {StatementNode::parse_function(tokens)?}
        );
        Ok(node)
    }

    fn parse_return(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let expr = Expression::parse(tokens, 0)?;
        match_token!(tokens.next()?, Token::EndStatement, "expected endstatement");
        Ok(StatementNode::Return { expr })
    }

    fn parse_declaration(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let identifier_name =
            match_token!(tokens.next()?, Token::Identifier(name) => {name}, "expected identifier");
        let literal = identifier_name.clone();

        let mut size: usize = 1;

        match_token!(tokens.peek(0)?,
            Token::OpenSquare => {
                tokens.next()?;
                size = match_token!(tokens.next()?,
                    Token::Int(val) => {
                        if *val <= 0 {
                            return Err(CompilationError::new("array size cant be negative or 0"))
                        } 
                        else { *val as usize } 
                    }, "expected int");
                match_token!(tokens.next()?, Token::ClosedSquare, "expected ]");
            },
            _ => {}
        );

        match_token!(tokens.next()?, Token::Equals, "expected equals");
        let expr = Expression::parse(tokens, 0)?;
        match_token!(tokens.next()?, Token::EndStatement, "expected endstatement");
        Ok(StatementNode::Declaration {
            expr,
            literal,
            size,
        })
    }

    fn parse_if(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let expr = Expression::parse(tokens, 0)?;
        match_token!(tokens.next()?, Token::OpenCurly, "expected scope");
        let scope = StatementNode::parse_scope(tokens)?;
        Ok(StatementNode::If {
            expr,
            scope: Box::new(scope),
        })
    }

    fn parse_function(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let identifier_name = match_token!(tokens.next()?, Token::Identifier(name) => {name}, "expected function name");
        let name = identifier_name.clone();
        match_token!(tokens.next()?, Token::OpenBracket, "expected (");
        let mut args: Vec<String> = Vec::new();
        while match_token!(tokens.next()?, "unexpected token in function definition",
        Token::ClosedBracket => {false},
        Token::Identifier(name) => {
            let name = name.clone();
            if let Token::Comma = tokens.peek(0)? {
                tokens.next()?;
            }
            args.push(name);
            true
        }) {}

        match_token!(tokens.peek(0)?,
         Token::OpenCurly => {
            tokens.next()?;
                let scope = StatementNode::parse_scope(tokens)?;
                Ok(StatementNode::Function {
                    name,
                    scope: Box::new(scope),
                    args,
                })
        }, "expected scope")
    }

    fn parse_scope(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let mut statements: Vec<StatementNode> = Vec::new();
        while match_token!(tokens.peek(0)?,
        Token::ClosedCurly => {
            tokens.next()?;
            return Ok(StatementNode::Scope { statements });
        },
        _ => {
            let stmt = StatementNode::parse(tokens)?;
            statements.push(stmt);
            true
        }) {}
        Err(CompilationError::new("unclosed scope"))
    }

    fn parse_while(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        let expr = Expression::parse(tokens, 0)?;
        match_token!(tokens.next()?,
            Token::OpenCurly => {
                let scope = StatementNode::parse_scope(tokens)?;
                Ok(StatementNode::While {
                    expr,
                    scope: Box::new(scope),
                })
            },
            _ => {Err(CompilationError::new("expected scope"))}
        )
    }

    fn parse_assignment(tokens: &mut Tokens, var_name: String) -> Result<Self, CompilationError> {
        let mut index_expr: Option<Expression> = None;

        match_token!(tokens.peek(0)?,
        Token::OpenSquare => {
            tokens.next()?;
            index_expr = Some(Expression::parse(tokens, 0)?); 
            match_token!(tokens.next()?, Token::ClosedSquare, "expected ]");
        },
        _=>{});
        
        match_token!(tokens.next()?, Token::Equals => {
                let expr = Expression::parse(tokens, 0)?;
                match_token!(tokens.next()?,
                    Token::EndStatement => {Ok(StatementNode::Assignment {
                        literal: var_name,
                        expr,
                        index_expr
                    })},
                "expected end statement")
            },
            _ => {Err(CompilationError::new("expected equals "))}
        )
    }

    fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            StatementNode::Declaration { literal, expr , size} => {
                println!("DECLARED {literal}");
                expr.to_asm(parsing_context)?;
                let name = literal.clone();
                parsing_context.push_on_stack("rdi");
                if parsing_context.add_var(name).is_none() {
                    return Err(CompilationError::new(
                        format!("Variable {literal} already exists").as_str(),
                    ));
                }
                for _ in 0..(size - 1)
                {
                    parsing_context.push_on_stack("rdi");
                }
            }
            StatementNode::Return { expr } => {
                if parsing_context.stack_pointers().len() > 1 {
                    expr.to_asm(parsing_context)?;
                    parsing_context.clear_current_stack();
                    parsing_context.push_line("    ret");
                } else {
                    expr.to_asm(parsing_context)?;
                    parsing_context.push_line("    mov rax, 60");
                    parsing_context.push_line("    syscall");
                }
            }
            StatementNode::Scope { statements } => {
                parsing_context.push_scope();
                for stmt in statements {
                    stmt.to_asm(parsing_context)?;
                }
                parsing_context.pop_scope();
            }
            StatementNode::If { expr, scope } => {
                let label = parsing_context.new_label();
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    cmp rdi, 0");
                parsing_context.push_line(format!("    je {label}").as_str());
                scope.to_asm(parsing_context)?;
                parsing_context.push_line(format!("{label}:").as_str());
            }
            StatementNode::While { expr, scope } => {
                let true_label = parsing_context.new_label();
                let false_label = parsing_context.new_label();

                parsing_context.add_loop_exit_label(false_label.clone());

                parsing_context.push_line(format!("{true_label}:").as_str());
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    cmp rdi, 0");
                parsing_context.push_line(format!("    je {false_label}").as_str());
                scope.to_asm(parsing_context)?;
                parsing_context.push_line(format!("    jmp {true_label}").as_str());
                parsing_context.push_line(format!("{false_label}:").as_str());

                parsing_context.pop_loop_exit_label();
            }
            StatementNode::Assignment { literal, expr , index_expr} => {
                let name = literal.clone();
                if let Some(var) = parsing_context.get_var(&name) {
                    match index_expr {
                        Some(index_expr) => {
                            index_expr.to_asm(parsing_context)?;
                            parsing_context.push_line("    mov rax, rdi");
                            expr.to_asm(parsing_context)?;
                            parsing_context
                                .push_line(format!("    mov [rbp - {} + rax * 4], rdi", var.stack_position).as_str())
                        }
                        None => {
                            expr.to_asm(parsing_context)?;
                            parsing_context
                                .push_line(format!("    mov [rbp - {}], rdi", var.stack_position).as_str())
                        }
                    }

                } else {
                    return Err(CompilationError::new(
                        format!("Variable {literal} doesnt exist").as_str(),
                    ));
                }
            }
            StatementNode::Break => {
                if let Some(label) = parsing_context.current_loop_exit_label() {
                    parsing_context.push_line(format!("    jmp {}", label).as_str())
                } else {
                    return Err(CompilationError::new("break without label"));
                }
            }
            StatementNode::Print { expr } => {
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rax, 1");
                parsing_context.push_line("    mov rsi, rdi");
                parsing_context.push_line("    mov rdi, 1");
                parsing_context.push_line("    mov rdx, 1");
                parsing_context.push_line("    syscall");
            }
            StatementNode::Function { name, scope, args } => {
                if parsing_context.add_function_name(name.clone()) {
                    parsing_context.add_stack_pointer();
                    parsing_context.push_line(format!("{}:", name).as_str());
                    parsing_context.push_on_stack("rbp");
                    parsing_context.push_line("    mov rbp, rsp");

                    parsing_context.push_scope();
                    for i in 0..args.len() {
                        if let Some(arg) = args.get(i) {
                            let offset = -(args.len() as i64 - i as i64 + 1) * 8;
                            parsing_context.add_offset_var(arg.clone(), offset);
                        }
                    }

                    scope.to_asm(parsing_context)?;
                    parsing_context.pop_scope();
                    parsing_context.pop_stack_pointer();
                } else {
                    return Err(CompilationError::new(
                        format!("function {} already exists", name).as_str(),
                    ));
                }
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
