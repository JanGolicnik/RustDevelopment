use super::super::super::tokenization::{Tokens, token::Token};
use super::super::{ParsingContext, CompilationError};
use super::expression::Expression;
use crate::match_token;

pub enum StatementNode {
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
        len: Expression,
    },
    Read {
        ptr: Expression,
        len: Expression,
    },
    Function {
        name: String,
        scope: Box<StatementNode>,
        args: Vec<String>,
    },
}

impl StatementNode {
    pub fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
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
                match_token!(tokens.next()?, Token::Comma, "expected comma");
                let len = Expression::parse(tokens, 0)?;
                match tokens.next()? {
                    Token::EndStatement => StatementNode::Print { expr, len },
                    _ => {
                        return Err(CompilationError::new("Expected ;"));
                    }
                }
            },
            Token::Read => {
                let ptr = Expression::parse(tokens, 0)?;
                match_token!(tokens.next()?, Token::Comma, "expected comma");
                let len = Expression::parse(tokens, 0)?;
                match tokens.next()? {
                    Token::EndStatement => StatementNode::Read { ptr, len },
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

    pub fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
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
            StatementNode::Print { expr, len } => {
                expr.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rsi, rdi");
                len.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rdx, rdi");
                parsing_context.push_line("    mov rax, 1");
                parsing_context.push_line("    mov rdi, 1");
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
            StatementNode::Read { ptr, len } => {
                ptr.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rsi, rdi");
                len.to_asm(parsing_context)?;
                parsing_context.push_line("    mov rdx, rdi");
                parsing_context.push_line("    mov rax, 0");
                parsing_context.push_line("    mov rdi, 1");
                parsing_context.push_line("    syscall");
            }
            
        }
        Ok(())
    }
}
