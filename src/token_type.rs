use std::fmt; 
#[derive(Debug)]
enum TokenType {
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

struct Token<T: fmt::Display> {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: T,
    pub line: usize,
}
impl<T: fmt::Display> Token<T> {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.r#type, self.lexeme, self.literal)
    }
}
