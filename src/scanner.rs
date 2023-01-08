use crate::token_type::*;

pub struct Scanner {
    source: String,
    view: String, // Current char(s) being scanned
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            view: "".to_string(),
            tokens: vec!(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        for c in self.source.chars() {
            self.start = self.current;
            self.view.push(c);
            if let Some(token) = Self::scan_token(&mut self.view, self.line) {
                self.tokens.push(token);
                self.view = "".to_string();
            }
        }
        self.tokens.push(
            Token { 
                r#type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: self.line,
            }
        );
    }

    fn scan_token(view: &mut String, line: usize) -> Option<Token> {
        match view.as_str() {
           "(" => Some(Self::create_token(TokenType::LeftParen, view, line)),
           ")" => Some(Self::create_token(TokenType::RightParen, view, line)),
           "{" => Some(Self::create_token(TokenType::LeftBrace, view, line)),
           "}" => Some(Self::create_token(TokenType::RightBrace, view, line)),
           "," => Some(Self::create_token(TokenType::Comma, view, line)),
           "." => Some(Self::create_token(TokenType::Dot, view, line)),
           "-" => Some(Self::create_token(TokenType::Minus, view, line)),
           "+" => Some(Self::create_token(TokenType::Plus, view, line)),
           ";" => Some(Self::create_token(TokenType::Semicolon, view, line)),
           "*" => Some(Self::create_token(TokenType::Star, view, line)),
           _ => None
        }
    }

    fn create_token(r#type: TokenType, view: &mut String, line: usize) -> Token {
        Self::create_token_with_literal(r#type, view, line, None)
    }

    fn create_token_with_literal(r#type: TokenType, view: &mut String, line: usize, literal: Option<Literal>) -> Token {
        Token {
            r#type,
            lexeme: view.to_owned(),
            literal,
            line,
        }
    }
}
