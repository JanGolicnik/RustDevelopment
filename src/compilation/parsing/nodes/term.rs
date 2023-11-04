use super::expression::Expression;
use super::super::ParsingContext;
use super::super::CompilationError;
use super::super::super::tokenization::{Tokens, token::Token};
use crate::match_token;

#[derive(Debug)]
pub enum IdentifierValueType{
    Reference,
    Dereference,
    Value
}

#[derive(Debug)]
pub enum Term {
    Int(i32),
    Identifier{name: String, index_expr: Option<Box<Expression>>, is_ref: IdentifierValueType},
    String(String),
    FunctionCall(String, Vec<Expression>),
}


impl Term {
    pub fn parse(tokens: &mut Tokens) -> Result<Self, CompilationError> {
        match_token!(tokens.peek(0)?, "weird term",
            Token::Int(int_token_val) => { 
                let int_token_val = *int_token_val;
                tokens.next()?;
                Ok(Term::Int(int_token_val))
            },

            Token::String(val) => { 
                let val = val.clone(); 
                tokens.next()?; 
                Ok(Term::String( val ))
            },
                
            Token::Identifier(_) | Token::And | Token::Star => {
                Term::parse_identifier(tokens)
            }
        )
    }

    fn parse_identifier(tokens: &mut Tokens) -> Result<Self, CompilationError>{
        let is_ref = match_token!(tokens.peek(0)?,
            Token::And => {tokens.next()?; IdentifierValueType::Reference}, 
            Token::Star => {tokens.next()?; IdentifierValueType::Dereference}, 
            _ => {IdentifierValueType::Value}
        );
        
        let name = match_token!(tokens.next()?, "expected identifier. How did you get here?",
            Token::Identifier(name) => {
                name.clone()
            }
        );

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
                
                if matches!(is_ref, IdentifierValueType::Dereference | IdentifierValueType::Reference) {
                    return Err(CompilationError::new("cannot reference a function"));
                }

                Ok(Term::FunctionCall(name, expressions))
            },
            Token::OpenSquare => {
                tokens.next()?;
                let expr = Expression::parse(tokens, 0)?;
                match_token!(tokens.next()?, 
                    Token::ClosedSquare => {
                        Ok(Term::Identifier { name, index_expr: Some(Box::new(expr)), is_ref })
                    }, "expected ]")
            },
            _ => {
                Ok(Term::Identifier{name, index_expr: None, is_ref})
            }
        )
    }

    pub fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
        match self {
            Term::Int(val) => {
                parsing_context.push_line(format!("    mov rdi, {}", val).as_str());
                Ok(())
            }
            Term::Identifier{name, index_expr, is_ref} => {
                if let Some(var) = parsing_context.get_var(name) {
                    match index_expr {
                        Some(index_expr) => {
                            index_expr.to_asm(parsing_context)?;
                            parsing_context.push_line("    mov rax, rdi");

                            match *is_ref{
                                IdentifierValueType::Dereference => {
                                    let sign = if var.stack_position < 0 { "+" } else { "-" };
                                    parsing_context.push_line(
                                        format!("    mov rdi, [rbp {} {} + rax * 4]", sign, var.stack_position.abs())
                                            .as_str());
                                    parsing_context.push_line("mov rdi, [rdi]");
                                },
                                IdentifierValueType::Reference => {
                                    parsing_context.push_line("    mov rdi, rbp");
                                    parsing_context.push_line(format!("    {} rdi, {}", if var.stack_position.is_negative() {"add"} else {"sub"}, var.stack_position).as_str());
                                    parsing_context.push_line("    mul rax, 4");
                                    parsing_context.push_line("    add rdi, rax");
                                },
                                IdentifierValueType::Value => {
                                    let sign = if var.stack_position < 0 { "+" } else { "-" };
                                    parsing_context.push_line(
                                        format!("    mov rdi, [rbp {} {} + rax * 4]", sign, var.stack_position.abs())
                                            .as_str(),
                                    );
                                }
                            }
                        }
                        None => {
                            match *is_ref{
                                IdentifierValueType::Dereference => {
                                    let sign = if var.stack_position < 0 { "+" } else { "-" };
                                    parsing_context.push_line(
                                        format!("    mov rdi, [rbp {} {}]", sign, var.stack_position.abs())
                                            .as_str());
                                    parsing_context.push_line("mov rdi, [rdi]");
                                },
                                IdentifierValueType::Reference => {
                                     parsing_context.push_line("    mov rdi, rbp");
                                    parsing_context.push_line(format!("    {} rdi, {}", if var.stack_position.is_negative() {"add"} else {"sub"}, var.stack_position).as_str());
                                },
                                IdentifierValueType::Value => {
                                    let sign = if var.stack_position < 0 { "+" } else { "-" };
                                parsing_context.push_line(
                                    format!("    mov rdi, [rbp {} {}]", sign, var.stack_position.abs())
                                        .as_str(),
                                );
                                }
                            }
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