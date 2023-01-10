use crate::token_type::*;
use crate::lox;

pub struct Scanner {
    view: String, // Current char(s) being scanned
    chars: std::iter::Peekable<std::vec::IntoIter<char>>,
    pub tokens: Vec<Token>,
    pub errors: Vec<lox::Error>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            chars: source.chars().collect::<Vec<_>>().into_iter().peekable(),
            ..Default::default()
        }
    }

    pub fn scan_tokens(source: String) -> Result<Vec<Token>, Vec<lox::Error>> {
        let mut scanner: Scanner = Self::new(source);
        let mut errors = vec!();
        while scanner.chars.peek().is_some() {
            let c = scanner.chars.next().unwrap();
            scanner.start = scanner.current;
            scanner.view.push(c);
            scanner.scan_token();
            // match scanner.scan_token() {
            //     Ok(token) => {
            //         scanner.tokens.push(token);
            //         scanner.view = "".to_string();
            //     }
            //     Err(e) => {
            //         errors.push(e);
            //     }
            // }
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
        Err(errors)
    }

    fn scan_token(&mut self) {
        match self.view.as_str() {
            "(" => self.tokens.push(Self::create_token(TokenType::LeftParen, &mut self.view, self.line)),
            ")" => self.tokens.push(Self::create_token(TokenType::RightParen, &mut self.view, self.line)),
            "{" => self.tokens.push(Self::create_token(TokenType::LeftBrace, &mut self.view, self.line)),
            "}" => self.tokens.push(Self::create_token(TokenType::RightBrace, &mut self.view, self.line)),
            "," => self.tokens.push(Self::create_token(TokenType::Comma, &mut self.view, self.line)),
            "." => self.tokens.push(Self::create_token(TokenType::Dot, &mut self.view, self.line)),
            "-" => self.tokens.push(Self::create_token(TokenType::Minus, &mut self.view, self.line)),
            "+" => self.tokens.push(Self::create_token(TokenType::Plus, &mut self.view, self.line)),
            ";" => self.tokens.push(Self::create_token(TokenType::Semicolon, &mut self.view, self.line)),
            "*" => self.tokens.push(Self::create_token(TokenType::Star, &mut self.view, self.line)),
            "!" => {
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.tokens.push(Self::create_token(TokenType::BangEqual, &mut self.view, self.line))
                    },
                    _ => self.tokens.push(Self::create_token(TokenType::Bang, &mut self.view, self.line))
                }
            }
            "=" => {
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.tokens.push(Self::create_token(TokenType::EqualEqual, &mut self.view, self.line))
                    },
                    _ => self.tokens.push(Self::create_token(TokenType::Equal, &mut self.view, self.line))
                }
            }
            "<" => {
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.tokens.push(Self::create_token(TokenType::LessEqual, &mut self.view, self.line))
                    },
                    _ => self.tokens.push(Self::create_token(TokenType::Less, &mut self.view, self.line))
                }
            }
            ">" => {
                match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.tokens.push(Self::create_token(TokenType::GreaterEqual, &mut self.view, self.line))
                    },
                    _ => self.tokens.push(Self::create_token(TokenType::Greater, &mut self.view, self.line))
                }
            }
            "/" => {
                match self.chars.peek() {
                    Some('/') => {
                        self.chars.next();
                        while self.chars.peek() != Some(&'\n') && self.chars.peek() != None {
                            self.chars.next();
                        };
                    },
                    _ => self.tokens.push(Self::create_token(TokenType::Slash, &mut self.view, self.line))
                }
            }
            _ => {
                self.errors.push(
                    lox::Error {
                        line: self.line,
                        message: "Unexpected character.".to_string()
                    }
                    );
            }
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
            view: "".to_string(),
            chars: "".to_string().chars().collect::<Vec<_>>().into_iter().peekable(),
            tokens: vec!(),
            errors: vec!(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Iterator for Scanner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}
