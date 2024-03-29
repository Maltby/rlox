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
        self.expect_token(
            token_type::TokenType::Semicolon,
            "Expected variable declaration to end with a semicolon".to_string(),
        )?;
        Ok(stmt::Stmt::VarDec(stmt::VarDec { name, expression }))
    }

    fn statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Print => self.print_statement(),
            token_type::TokenType::LeftBrace => self.block_statement(),
            token_type::TokenType::If => self.if_statement(),
            token_type::TokenType::While => self.while_statement(),
            token_type::TokenType::For => self.for_statement(),
            _ => self.expression_statement(),
        }
    }

    fn for_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume 'for'
        self.expect_token(
            token_type::TokenType::LeftParen,
            "Expected '(' after 'for'".to_string(),
        )?;
        let initializer = match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Semicolon => None,
            token_type::TokenType::Var => Some(self.var_declaration()?),
            _ => Some(self.expression_statement()?),
        };
        let condition = match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::Semicolon => None,
            _ => Some(self.expression()?),
        };
        self.expect_token(
            token_type::TokenType::Semicolon,
            "Expected ';' after for condition".to_string(),
        )?;
        let increment = match self.tokens.peek().unwrap().r#type {
            token_type::TokenType::RightParen => None,
            _ => Some(self.expression()?),
        };
        self.expect_token(
            token_type::TokenType::RightParen,
            "Expected ')' after for increment expression".to_string(),
        )?;
        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = stmt::Stmt::Block(stmt::Block {
                statements: vec![
                    body,
                    stmt::Stmt::Expr(stmt::Expr {
                        expression: increment,
                    }),
                ],
            })
        };
        let condition = match condition {
            Some(condition) => condition,
            None => expr::Expr::Literal(expr::Literal::Bool(true)),
        };
        body = stmt::Stmt::While(stmt::While {
            condition,
            body: Box::new(body),
        });
        if let Some(initializer) = initializer {
            body = stmt::Stmt::Block(stmt::Block {
                statements: vec![initializer, body],
            });
        };
        Ok(body)
    }

    fn while_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume 'while'
        self.expect_token(
            token_type::TokenType::LeftParen,
            "Expected '(' after 'while'".to_string(),
        )?;
        let condition = self.expression()?;
        self.expect_token(
            token_type::TokenType::RightParen,
            "Expected ')' after while condition".to_string(),
        )?;
        let body = self.statement()?;
        Ok(stmt::Stmt::While(stmt::While {
            condition,
            body: Box::new(body),
        }))
    }

    fn expect_token(
        &mut self,
        expected: token_type::TokenType,
        err_msg: String,
    ) -> Result<token_type::Token, ParseError> {
        match self.tokens.peek() {
            Some(token) => match &token.r#type {
                x if *x == expected => Ok(self.tokens.next().unwrap()),
                _ => Err(ParseError {
                    description: err_msg,
                }),
            },
            None => Err(ParseError {
                description: format!("Expected {} but found nothing", expected),
            }),
        }
    }

    fn if_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume 'if'
        self.expect_token(
            token_type::TokenType::LeftParen,
            "Expected '(' after 'if'".to_string(),
        )?;
        let condition = self.expression()?;
        self.expect_token(
            token_type::TokenType::RightParen,
            "Expected ')' after if condition".to_string(),
        )?;
        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if let token_type::TokenType::Else = self.tokens.peek().unwrap().r#type {
            self.tokens.next(); // consume 'else'
            else_branch = Some(Box::new(self.statement()?));
        };
        Ok(stmt::Stmt::If(stmt::If {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume 'print'
        let value = self.expression()?;
        self.expect_token(
            token_type::TokenType::Semicolon,
            "Expected print statement to end with a semicolon".to_string(),
        )?;
        Ok(stmt::Stmt::Print(stmt::Print { expression: value }))
    }

    fn block_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        self.tokens.next(); // consume '{'
        let mut statements = vec![];
        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::RightBrace => {
                    self.tokens.next(); // consume '}'
                    return Ok(stmt::Stmt::Block(stmt::Block { statements }));
                }
                token_type::TokenType::Eof => {
                    return Err(ParseError {
                        description: "Expected '}' after block".to_string(),
                    })
                }
                _ => statements.push(self.declaration()?),
            }
        }
        return Err(ParseError {
            description: "Expected '}' after block".to_string(),
        });
    }

    fn expression_statement(&mut self) -> Result<stmt::Stmt, ParseError> {
        let value = self.expression()?;
        self.expect_token(
            token_type::TokenType::Semicolon,
            "Expected expression statement to end with a semicolon".to_string(),
        )?;
        Ok(stmt::Stmt::Expr(stmt::Expr { expression: value }))
    }

    fn expression(&mut self) -> Result<expr::Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<expr::Expr, ParseError> {
        let expr = self.or()?;
        if let Some(token) = self.tokens.peek() {
            match token.r#type {
                token_type::TokenType::Equal => {
                    self.tokens.next(); // consume '='
                    let value = self.assignment()?;
                    match expr {
                        expr::Expr::Variable(variable) => {
                            return Ok(expr::Expr::Assign(Box::new(expr::Assign {
                                name: variable.name,
                                value,
                            })))
                        }
                        _ => {
                            return Err(ParseError {
                                description: "Invalid assignment target".to_string(),
                            })
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = self.and()?;
        while let token_type::TokenType::Or = self.tokens.peek().unwrap().r#type {
            let operator = self.tokens.next().unwrap();
            let right = self.and()?;
            expr = expr::Expr::Logical(Box::new(expr::Logical {
                operator,
                left: expr,
                right,
            }));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = self.equality()?;
        while let token_type::TokenType::And = self.tokens.peek().unwrap().r#type {
            let operator = self.tokens.next().unwrap();
            let right = self.equality()?;
            expr = expr::Expr::Logical(Box::new(expr::Logical {
                operator,
                left: expr,
                right,
            }));
        }
        Ok(expr)
    }

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

    fn comparison(&mut self) -> Result<expr::Expr, ParseError> {
        let mut expr = self.term()?;
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
