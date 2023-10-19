use crate::parse_error::ParseError;
use crate::tokenization::Token;

struct ReturnNode {
    expr: Expression,
}

impl ReturnNode {
    fn parse(tokens: &[Token], index: &mut usize) -> Result<Self, ParseError> {
        *index += 1;
        Ok(ReturnNode {
            expr: Expression::parse(tokens, index)?,
        })
    }

    fn to_asm(&self) -> String {
        let val = match self.expr {
            Expression::Literal(val) => val,
            _ => 0,
        };
        format!("    mov rax, 60\n    mov rdi, {}\n    syscall", val)
    }
}

enum Expression {
    Literal(i32),
}

impl Expression {
    fn parse(tokens: &[Token], index: &mut usize) -> Result<Self, ParseError> {
        if let Some(lit_token) = tokens.get(*index) {
            match lit_token {
                Token::Int(int_token_val) => {
                    *index += 1;
                    if let Some(semi_token) = tokens.get(*index) {
                        match semi_token {
                            Token::Semicolon => Ok(Expression::Literal(*int_token_val)),
                            _ => Err(ParseError::new("Missing semicolon")),
                        }
                    } else {
                        Err(ParseError::new("Missing token"))
                    }
                }
                _ => Err(ParseError::new("Token should be literal")),
            }
        } else {
            Err(ParseError::new("Missing token"))
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<String, ParseError> {
    let mut index = 0;

    let token = &tokens[index];
    match *token {
        Token::Return => {
            let root_node = ReturnNode::parse(&tokens, &mut index)?;

            let mut output = "global _start:\n_start:\n".to_string();
            output += &root_node.to_asm();

            Ok(output)
        }
        _ => Err(ParseError::new("Unexpected token")),
    }

    // Err(ParseError::new("parsing isnt written :/"))
}
