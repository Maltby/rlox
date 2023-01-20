use crate::token_type;
use crate::expr;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    description: String,
}
impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.description)
    }
}

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<token_type::Token>>,
}
impl Parser {
    // BNF: expression -> equality ;
    fn expression(&mut self) -> Result<expr::Expr, ParseError> {
        self.equality() 
    }

    // BNF: equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };
        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::BangEqual | token_type::TokenType::EqualEqual => {
                    let operator = self.tokens.next().unwrap();
                    match self.comparison() {
                        Ok(right) => {
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right
                                    }
                                    ));
                        },
                        Err(e) => return Err(e),
                    }
                },
                _ => break,
            }
        }
        Ok(expr)
    }

    // BNF: comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };
        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Greater | token_type::TokenType::GreaterEqual | token_type::TokenType::Less | token_type::TokenType::LessEqual => {
                    let operator = self.tokens.next().unwrap();
                    match self.term() {
                        Ok(right) => {
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right,
                                    }
                                    ));
                        },
                        Err(e) => return Err(e),
                    }
                },
                _ => break,
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Minus | token_type::TokenType::Plus => {
                    let operator = self.tokens.next().unwrap();
                    match self.factor() {
                        Ok(right) => {
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right,
                                    }
                                    ));
                        },
                        Err(e) => return Err(e),
                    }
                },
                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };
        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Slash | token_type::TokenType::Star => {
                    let operator = self.tokens.next().unwrap();
                    match self.unary() {
                        Ok(right) => {
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right,
                                    }
                                    ));
                        },
                        Err(e) => return Err(e),
                    }
                },
                _ => break,
            }
        }
        Ok(expr)
    }

    // BNF: unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<expr::Expr, ParseError> {
        if let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Bang | token_type::TokenType::Minus => {
                    let operator = self.tokens.next().unwrap();
                    match self.unary() {
                        Ok(right) => {
                            return Ok(expr::Expr::Unary(Box::new(
                                        expr::Unary {
                                            operator,
                                            right,
                                        })));
                        },
                        Err(e) => return Err(e)
                    }
                },
                _ => {
                    return Err(ParseError{description: "Expected Bang or Minus token".to_string()})
                },
            }
        }
        self.primary()
    }

    // BNF: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<expr::Expr, ParseError> {
        let token = match self.tokens.next() {
            Some(token) => token,
            None => panic!("Expected a token")
        };
        match token.r#type {
            token_type::TokenType::False => Ok(expr::Expr::Literal(expr::Literal::Bool(false))),
            token_type::TokenType::True => Ok(expr::Expr::Literal(expr::Literal::Bool(true))),
            token_type::TokenType::Nil => Ok(expr::Expr::Literal(expr::Literal::Nil)),
            token_type::TokenType::Number => {
                match token.literal.as_ref().unwrap() {
                    token_type::Literal::Number(x) => Ok(expr::Expr::Literal(expr::Literal::Number(*x))),
                    _ => Err(ParseError{description: "Number token did not contain a Number".to_string()})
                }
            },
            token_type::TokenType::String => {
                match token.literal.as_ref().unwrap() {
                    token_type::Literal::String(x) => Ok(expr::Expr::Literal(expr::Literal::String(x.clone()))),
                    _ => Err(ParseError{description: "String token did not contain a String".to_string()})
                }
            }
            token_type::TokenType::LeftParen => {
                let expr = self.expression();
                if self.tokens.peek().unwrap().r#type == token_type::TokenType::RightParen {
                    self.tokens.next(); // consume RightParen
                    Ok(expr::Expr::Grouping(Box::new(
                            expr::Grouping {
                                expression: expr
                            })))
                } else {
                    Err(ParseError{description: "Grouping did not end in right paren".to_string()})
                }
            },
            _ => Err(ParseError{description: format!("Unexpected token: {}", token)})
        }
    }
}
