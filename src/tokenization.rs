use unicode_segmentation::UnicodeSegmentation;

use crate::compilation_error::CompilationError;

#[derive(Debug)]
pub enum Token {
    Return,
    Int(i32),
    Identifier(String),
    EndStatement,
    Equals,
    Declaration,
    Addition,
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
    matches!(grapheme, ";" | " " | "=" | "\n" | "+")
}

fn tokenize_separator(grapheme: &str) -> Option<Token> {
    println!("{}", grapheme);
    match grapheme {
        ";" => Some(Token::EndStatement),
        "=" => Some(Token::Equals),
        "+" => Some(Token::Addition),
        _ => None,
    }
}
