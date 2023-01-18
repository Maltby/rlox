use std::fmt; 
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single and double char tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot,
    Minus, Plus,
    Semicolon, Slash,
    Star,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Special
    Eof,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(x) => write!(f, "{}", x),
            Literal::String(x) => write!(f, "{}", x),
        }
    }
}

pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>, // could be either a string or a number
    pub line: usize,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "{}", literal),
            None => write!(f, "{}", self.lexeme),
        }
    }
}
