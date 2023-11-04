#[derive(Debug)]
pub enum Token {
    EndLine,
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
    OpenSquare,
    ClosedSquare,
    If,
    LessThan,
    GreaterThan,
    While,
    Break,
    Print,
    String(String),
    Function,
    Comma,
    And,
    Read,
}

pub struct OperatorInfo(pub usize, pub bool);

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::EndLine => Token::EndLine,
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
            Token::OpenSquare => Token::OpenSquare,
            Token::ClosedSquare => Token::ClosedSquare,
            Token::If => Token::If,
            Token::LessThan => Token::LessThan,
            Token::GreaterThan => Token::GreaterThan,
            Token::While => Token::While,
            Token::Break => Token::Break,
            Token::Print => Token::Print,
            Token::String(val) => Token::String(val.clone()),
            Token::Function => Token::Function,
            Token::Comma => Token::Comma,
            Token::And => Token::And,
            Token::Read => Token::Read,
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

