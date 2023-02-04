use crate::expr;
use crate::stmt;
use crate::token_type;
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
    pub fn parse(tokens: Vec<token_type::Token>) -> Result<Vec<stmt::Stmt>, ParseError> {
        let mut parser = Parser {
            tokens: tokens.into_iter().peekable(),
        };
        let mut stmts = vec![];
        while let Some(token) = parser.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Eof => return Ok(stmts),
                _ => stmts.push(parser.declaration()?),
            }
        }
        Err(ParseError {
            description: "No Eof found".to_string(),
        })
    }

    fn declaration(&mut self) -> Result<stmt::Stmt, ParseError> {
        match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Var => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume 'var'
        let name = self.tokens.next().unwrap();
        let expression = match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Equal => {
                self.tokens.next(); // consume '='
                Some(self.expression()?)
            }
            _ => None,
        };
        match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Semicolon => {
                self.tokens.next();
            }
            _ => {
                return Err(ParseError {
                    description: "Expected variable declaration to end with a semicolon"
                        .to_string(),
                })
            }
        }
        Ok(stmt::Stmt::Var(Box::new(stmt::Var { name, expression })))
    }

    // BNF: statement -> exprStmt | printStmt ;
    fn statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Print => self.print_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume 'print'
        let value = self.expression()?;
        if let Some(token) = self.tokens.peek() {
            return match token.r#type {
                token_type::TokenType::Semicolon => {
                    self.tokens.next(); // consume semicolon
                    Ok(stmt::Stmt::Print(Box::new(stmt::Print {
                        expression: value,
                    })))
                }
                _ => Err(ParseError {
                    description: "Expected statement to end with a semicolon".to_string(),
                }),
            };
        }
        Err(ParseError {
            description: "Expected statement to end with a semicolon".to_string(),
        })
    }

    fn expression_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        let value = self.expression()?;
        match self.tokens.peek() {
            Some(token) => {
                match token.r#type {
                    token_type::TokenType::Semicolon => {
                        self.tokens.next(); // consume semicolon
                        Ok(stmt::Stmt::Expr(Box::new(stmt::Expr { expression: value })))
                    }
                    _ => Err(ParseError {
                        description: "Expected statement to end with a semicolon".to_string(),
                    }),
                }
            }
            None => Err(ParseError {
                description: "Expected statement to end with a semicolon".to_string(),
            }),
        }
    }

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
                            expr = expr::Expr::Binary(Box::new(expr::Binary {
                                left: expr,
                                operator,
                                right,
                            }));
                        }
                        Err(e) => return Err(e),
                    }
                }
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
                token_type::TokenType::Greater
                | token_type::TokenType::GreaterEqual
                | token_type::TokenType::Less
                | token_type::TokenType::LessEqual => {
                    let operator = self.tokens.next().unwrap();
                    match self.term() {
                        Ok(right) => {
                            expr = expr::Expr::Binary(Box::new(expr::Binary {
                                left: expr,
                                operator,
                                right,
                            }));
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };
        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Minus | token_type::TokenType::Plus => {
                    let operator = self.tokens.next().unwrap();
                    match self.factor() {
                        Ok(right) => {
                            expr = expr::Expr::Binary(Box::new(expr::Binary {
                                left: expr,
                                operator,
                                right,
                            }));
                        }
                        Err(e) => return Err(e),
                    }
                }
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
                            expr = expr::Expr::Binary(Box::new(expr::Binary {
                                left: expr,
                                operator,
                                right,
                            }));
                        }
                        Err(e) => return Err(e),
                    }
                }
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
                            return Ok(expr::Expr::Unary(Box::new(expr::Unary {
                                operator,
                                right,
                            })));
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => {}
            }
        }
        self.primary()
    }

    // BNF: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<expr::Expr, ParseError> {
        let token = match self.tokens.next() {
            Some(token) => token,
            None => panic!("Expected a token"),
        };
        match token.r#type {
            token_type::TokenType::False => Ok(expr::Expr::Literal(expr::Literal::Bool(false))),
            token_type::TokenType::True => Ok(expr::Expr::Literal(expr::Literal::Bool(true))),
            token_type::TokenType::Nil => Ok(expr::Expr::Literal(expr::Literal::Nil)),
            token_type::TokenType::Number => match token.literal.as_ref().unwrap() {
                token_type::Literal::Number(x) => {
                    Ok(expr::Expr::Literal(expr::Literal::Number(*x)))
                }
                _ => Err(ParseError {
                    description: "Number token did not contain a Number".to_string(),
                }),
            },
            token_type::TokenType::String => match token.literal.as_ref().unwrap() {
                token_type::Literal::String(x) => {
                    Ok(expr::Expr::Literal(expr::Literal::String(x.clone())))
                }
                _ => Err(ParseError {
                    description: "String token did not contain a String".to_string(),
                }),
            },
            token_type::TokenType::Identifier => {
                Ok(expr::Expr::Variable(Box::new(expr::Variable {
                    name: token,
                })))
            }
            token_type::TokenType::LeftParen => {
                match self.expression() {
                    Ok(expr) => {
                        if self.tokens.peek().unwrap().r#type == token_type::TokenType::RightParen {
                            self.tokens.next(); // consume RightParen
                            Ok(expr::Expr::Grouping(Box::new(expr::Grouping {
                                expression: expr,
                            })))
                        } else {
                            Err(ParseError {
                                description: "Grouping did not end in right paren".to_string(),
                            })
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err(ParseError {
                description: format!("Unexpected token: {} ({})", token, token.r#type),
            }),
        }
    }
}
