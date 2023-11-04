use unicode_segmentation::UnicodeSegmentation;

use super::compilation_error::CompilationError;
use super::tokenization::tokens::Token;

pub mod tokens;

#[macro_export]
macro_rules! match_token {
    ($token:expr, $token_type:pat, $err:literal) => {
        let token__ = $token;
        if matches!(token__, $token_type) {
            token__
        } else {
            return Err(CompilationError::new($err));
        }
    };
    ($token:expr, $token_type:pat => $token_type_block:block, $err:literal) => {{
    let token__ = $token;
    match token__ {
        $token_type => $token_type_block,
        _ => return Err(CompilationError::new($err)),
    }
    }};
    ($token:expr, $err:literal, $( $token_type:pat => $token_type_block:block),*) => {{
        let token__ = $token;
        match token__ {
            $($token_type => $token_type_block,)*
            _ => return Err(CompilationError::new($err)),
        }
    }};
    ($token:expr, $( $token_type:pat => $token_type_block:block),*) => {{
        let token__ = $token;
        match token__ {
            $($token_type => $token_type_block,)*
        }
    }};
    ($token:expr, $token_type:pat => $token_type_block:block) => {{
        let token__ = $token;
        match token__ {
            $token_type => $token_type_block,
        }
    }};
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
        "read" => Some(Token::Read),
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
            | "["
            | "]"
            | "<"
            | ">"
            | "\""
            | ","
            | "&"
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
        "[" => Some(Token::OpenSquare),
        "]" => Some(Token::ClosedSquare),
        "<" => Some(Token::LessThan),
        ">" => Some(Token::GreaterThan),
        "," => Some(Token::Comma),
        "&" => Some(Token::And),
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
