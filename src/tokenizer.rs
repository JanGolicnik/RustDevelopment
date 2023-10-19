use unicode_segmentation::{Graphemes, UnicodeSegmentation};

#[derive(Debug)]
pub enum Token {
    Return,
    Int(i32),
    Semicolon,
}

pub fn tokenize(file: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut last_token_index = 0;

    let file_len = file.graphemes(true).count();

    for (index, grapheme) in file.graphemes(true).enumerate() {
        println!("{grapheme}");

        if index + 1 == file_len || is_separator(grapheme) {
            if let Some(token) = get_token(file[last_token_index..index].graphemes(true)) {
                tokens.push(token);
            } else {
                eprintln!(
                    "INVALID TOKEN AT {last_token_index} '{}'",
                    file[last_token_index..index + 1].graphemes(true).as_str()
                );
            }

            if let Some(token) = tokenize_separator(grapheme) {
                tokens.push(token);
            }

            last_token_index = index + 1;
        }
    }

    tokens
}

fn get_token(graphemes: Graphemes) -> Option<Token> {
    let chars = graphemes.as_str();

    println!("getting token {chars}");

    match graphemes.as_str() {
        "vrni" => Some(Token::Return),
        ";" => Some(Token::Semicolon),
        _ => str_to_int_token(chars),
    }
}

fn str_to_int_token(chars: &str) -> Option<Token> {
    match chars.parse::<i32>() {
        Ok(val) => Some(Token::Int(val)),
        Err(_) => None,
    }
}

fn is_separator(grapheme: &str) -> bool {
    matches!(grapheme, ";" | " " | "." | "(" | ")" | "-" | "{" | "}")
}

fn tokenize_separator(grapheme: &str) -> Option<Token> {
    match grapheme {
        ";" => Some(Token::Semicolon),
        _ => None,
    }
}
