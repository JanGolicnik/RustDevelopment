use super::super::super::tokenization::{token::{Token, OperatorInfo}, tokens::Tokens};
use super::super::{CompilationError, ParsingContext};
use crate::match_token;
use super::term::Term;

#[derive(Debug)]
pub enum Expression {
    Term(Term),
    Binary {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}


impl Expression {
    pub fn parse(tokens: &mut Tokens, min_precedence: usize) -> Result<Self, CompilationError> {
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

    pub fn to_asm(&self, parsing_context: &mut ParsingContext) -> Result<(), CompilationError> {
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
                parsing_context.push_line("    mov rax, rdi");
                right.to_asm(parsing_context)?;

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
                        parsing_context.push_line(format!("    jb {true_label}").as_str());
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
                        parsing_context.push_line(format!("    ja {true_label}").as_str());
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
