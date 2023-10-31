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
    LessThan,
    GreaterThan,
    While,
    Break,
    Print,
    String(String),
    Function,
    Comma,
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
            Token::LessThan => Token::LessThan,
            Token::GreaterThan => Token::GreaterThan,
            Token::While => Token::While,
            Token::Break => Token::Break,
            Token::Print => Token::Print,
            Token::String(val) => Token::String(val.clone()),
            Token::Function => Token::Function,
            Token::Comma => Token::Comma,
        }
    }
}

impl Token {
    pub fn get_operator_info(&self) -> Option<OperatorInfo> {
        match self {
            Token::GreaterThan | Token::LessThan | Token::Equals => Some(OperatorInfo(0, true)),
            Token::Plus => Some(OperatorInfo(1, true)),
            Token::Minus => Some(OperatorInfo(1, false)),
            Token::Star => Some(OperatorInfo(2, true)),
            Token::Slash => Some(OperatorInfo(2, false)),
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

    pub fn _tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn _reset(&mut self) {
        self.index = 0;
    }
}
pub fn tokenize(file: &String) -> Result<Tokens, CompilationError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut last_token_index = 0;

    let file_len = file.graphemes(true).count();
    let graphemes: Vec<&str> = file.graphemes(true).collect();
    let mut index = 0;
    while let Some(grapheme) = graphemes.get(index) {
        if index + 1 == file_len || is_separator(grapheme) {
            let word: String = file
                .graphemes(true)
                .skip(last_token_index)
                .take(index - last_token_index)
                .collect::<String>();

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

            if let Some(separator_token) = tokenize_separator(grapheme, &graphemes[..], &mut index)?
            {
                tokens.push(separator_token);
            }

            last_token_index = index + 1;
        }

        index += 1;
    }

    Ok(Tokens::new(tokens))
}

fn tokenize_word(word: &str) -> Option<Token> {
    match word {
        "return" => Some(Token::Return),
        "let" => Some(Token::Declaration),
        "if" => Some(Token::If),
        "true" => Some(Token::Int(1)),
        "false" => Some(Token::Int(0)),
        "while" => Some(Token::While),
        "break" => Some(Token::Break),
        "print" => Some(Token::Print),
        "fn" => Some(Token::Function),
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
        ";" | " "
            | "="
            | "\n"
            | "+"
            | "*"
            | "-"
            | "/"
            | "("
            | ")"
            | "{"
            | "}"
            | "<"
            | ">"
            | "\""
            | ","
    )
}

fn tokenize_separator(
    grapheme: &str,
    graphemes: &[&str],
    index: &mut usize,
) -> Result<Option<Token>, CompilationError> {
    Ok(match grapheme {
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
        "<" => Some(Token::LessThan),
        ">" => Some(Token::GreaterThan),
        "," => Some(Token::Comma),
        "\"" => {
            let mut chars: Vec<&str> = Vec::new();
            *index += 1;
            while let Some(gr) = graphemes.get(*index) {
                if *gr == "\"" {
                    let string = chars.iter().map(|s| s.to_string()).collect::<String>();
                    return Ok(Some(Token::String(string)));
                }
                chars.push(*gr);
                *index += 1;
            }

            return Err(CompilationError::new("unmatched \""));
        }
        _ => {
            return Ok(None);
        }
    })
}
