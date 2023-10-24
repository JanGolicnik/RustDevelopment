use unicode_segmentation::UnicodeSegmentation;

use crate::compilation_error::CompilationError;

#[derive(Debug)]
pub enum Token {
    Return,
    Int(i32),
    Identifier(String), // Change the representation to String
    EndStatement,
    Equals,
    Declaration,
    Plus,
    Star,
    Minus,
    Slash,
    OpenBracket,
    ClosedBracket,
    OpenCurly,
    ClosedCurly,
    If,
}

pub struct OperatorInfo(pub usize, pub bool);

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Return => Token::Return,
            Token::Int(value) => Token::Int(*value),
            Token::Identifier(identifier) => Token::Identifier(identifier.clone()), // Manually clone the String
            Token::EndStatement => Token::EndStatement,
            Token::Equals => Token::Equals,
            Token::Declaration => Token::Declaration,
            Token::Plus => Token::Plus,
            Token::Star => Token::Star,
            Token::Minus => Token::Minus,
            Token::Slash => Token::Slash,
            Token::OpenBracket => Token::OpenBracket,
            Token::ClosedBracket => Token::ClosedBracket,
            Token::OpenCurly => Token::OpenCurly,
            Token::ClosedCurly => Token::ClosedCurly,
            Token::If => Token::If,
        }
    }
}

impl Token {
    pub fn get_operator_info(&self) -> Option<OperatorInfo> {
        match self {
            Token::Plus => Some(OperatorInfo(0, true)),
            Token::Minus => Some(OperatorInfo(0, false)),
            Token::Star => Some(OperatorInfo(1, true)),
            Token::Slash => Some(OperatorInfo(1, false)),
            _ => None,
        }
    }
}

pub struct Tokens {
    tokens: Vec<Token>,
    index: usize,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Tokens { tokens, index: 0 }
    }

    pub fn next(&mut self) -> Result<&Token, CompilationError> {
        self.index += 1;

        if let Some(token) = self.tokens.get(self.index - 1) {
            Ok(token)
        } else {
            Err(CompilationError::new("Missing Token"))
        }
    }

    pub fn peek(&mut self, offset: usize) -> Result<&Token, CompilationError> {
        if let Some(token) = self.tokens.get(self.index + offset) {
            Ok(token)
        } else {
            Err(CompilationError::new("Missing Token"))
        }
    }

    pub fn _peek_back(&mut self, offset: usize) -> Result<&Token, CompilationError> {
        if let Some(token) = self.tokens.get(self.index - offset) {
            Ok(token)
        } else {
            Err(CompilationError::new("Missing Token"))
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
pub fn tokenize(file: &String) -> Result<Tokens, CompilationError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut last_token_index = 0;

    let file_len = file.graphemes(true).count();

    for (index, grapheme) in file.graphemes(true).enumerate() {
        println!("{} {}", index, grapheme);
        if index + 1 == file_len || is_separator(grapheme) {
            let graphemes: Vec<&str> = file
                .graphemes(true)
                .skip(last_token_index)
                .take(index - last_token_index)
                .collect();

            let word: String = graphemes.join("");
            if !word.is_empty() {
                if let Some(word_token) = tokenize_word(&word) {
                    tokens.push(word_token);
                } else {
                    return Err(CompilationError::new(
                        format!(
                            "invalid token {}",
                            file[last_token_index..index].graphemes(true).as_str()
                        )
                        .as_str(),
                    ));
                }
            }

            if let Some(separator_token) = tokenize_separator(grapheme) {
                tokens.push(separator_token);
            }

            last_token_index = index + 1;
        }
    }

    Ok(Tokens::new(tokens))
}

fn tokenize_word(word: &String) -> Option<Token> {
    let chars = word.as_str();
    println!("{}", word);
    match chars {
        "return" => Some(Token::Return),
        "let" => Some(Token::Declaration),
        "if" => Some(Token::If),
        "true" => Some(Token::Int(1)),
        "false" => Some(Token::Int(0)),
        _ => str_to_token(word),
    }
}

fn str_to_token(chars: &str) -> Option<Token> {
    match chars.parse::<i32>() {
        Ok(val) => Some(Token::Int(val)),
        Err(_) => Some(Token::Identifier(chars.to_string())),
    }
}

fn is_separator(grapheme: &str) -> bool {
    matches!(
        grapheme,
        ";" | " " | "=" | "\n" | "+" | "*" | "-" | "/" | "(" | ")" | "{" | "}"
    )
}

fn tokenize_separator(grapheme: &str) -> Option<Token> {
    println!("{}", grapheme);
    match grapheme {
        ";" => Some(Token::EndStatement),
        "=" => Some(Token::Equals),
        "+" => Some(Token::Plus),
        "*" => Some(Token::Star),
        "-" => Some(Token::Minus),
        "/" => Some(Token::Slash),
        "(" => Some(Token::OpenBracket),
        ")" => Some(Token::ClosedBracket),
        "{" => Some(Token::OpenCurly),
        "}" => Some(Token::ClosedCurly),
        _ => None,
    }
}
