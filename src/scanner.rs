use crate::token_type::*;
use crate::lox;

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
        let mut scanner = Scanner::default();
        scanner.source = source;
        scanner
    }

    pub fn scan_tokens(source: String) -> Result<Vec<Token>, Vec<lox::Error>> {
        let mut scanner: Scanner = Self::new(source);
        let mut errors = vec!();
        for c in scanner.source.chars() {
            scanner.start = scanner.current;
            scanner.view.push(c);
            match Self::scan_token(&mut scanner.view, scanner.line) {
                Ok(token) => {
                    scanner.tokens.push(token);
                    scanner.view = "".to_string();
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }
        scanner.tokens.push(
            Token { 
                r#type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: scanner.line,
            }
        );
        if errors.is_empty() {
            return Ok(scanner.tokens);
        }
        return Err(errors);
    }

    fn scan_token(view: &mut String, line: usize) -> Result<Token, lox::Error> {
        match view.as_str() {
            "(" => Ok(Self::create_token(TokenType::LeftParen, view, line)),
            ")" => Ok(Self::create_token(TokenType::RightParen, view, line)),
            "{" => Ok(Self::create_token(TokenType::LeftBrace, view, line)),
            "}" => Ok(Self::create_token(TokenType::RightBrace, view, line)),
            "," => Ok(Self::create_token(TokenType::Comma, view, line)),
            "." => Ok(Self::create_token(TokenType::Dot, view, line)),
            "-" => Ok(Self::create_token(TokenType::Minus, view, line)),
            "+" => Ok(Self::create_token(TokenType::Plus, view, line)),
            ";" => Ok(Self::create_token(TokenType::Semicolon, view, line)),
            "*" => Ok(Self::create_token(TokenType::Star, view, line)),
            _ => Err(
                lox::Error {
                    line,
                    message: "Unexpected character.".to_string()
                }
                )
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

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            source: "".to_string(),
            view: "".to_string(),
            tokens: vec!(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}
