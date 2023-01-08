use std::fmt; 
#[derive(Debug)]
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

pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>, // could be either a string or a number
    pub line: usize,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {:?}", self.r#type, self.lexeme, self.literal)
    }
}
