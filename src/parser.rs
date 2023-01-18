use crate::token_type;
use crate::expr;

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<token_type::Token>>,
}
// TODO: flatten logic of associated functions, they're a mess
impl Parser {
    // BNF: expression -> equality ;
    fn expression(&mut self) -> expr::Expr {
        self.equality() 
    }

    // BNF: equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> expr::Expr {
        let mut expr = self.comparison();
        loop {
            match self.tokens.peek() {
                Some(token) => {
                    match token.r#type {
                        token_type::TokenType::BangEqual | token_type::TokenType::EqualEqual => {
                            let operator = self.tokens.next().unwrap();
                            let right: expr::Expr = self.comparison();
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right
                                    }
                                    ));
                        },
                        _ => break,
                    }
                },
                None => break,
            };
        }
        expr
    }

    // BNF: comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> expr::Expr {
        let mut expr = self.term();
        loop {
            match self.tokens.peek() {
                Some(token) => {
                    match token.r#type {
                        token_type::TokenType::Greater | token_type::TokenType::GreaterEqual | token_type::TokenType::Less | token_type::TokenType::LessEqual => {
                            let operator = self.tokens.next().unwrap();
                            let right = self.term();
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right,
                                    }
                                    ));
                        },
                        _ => break,
                    }
                },
                None => break,
            }
        }
        expr
    }

    fn term(&mut self) -> expr::Expr {
        let mut expr = self.factor();
        loop {
            match self.tokens.peek() {
                Some(token) => {
                    match token.r#type {
                        token_type::TokenType::Minus | token_type::TokenType::Plus => {
                            let operator = self.tokens.next().unwrap();
                            let right = self.factor();
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right,
                                    }
                                    ));
                        },
                        _ => break,
                    }
                },
                None => break,
            }
        }
        expr
    }

    fn factor(&mut self) -> expr::Expr {
        let mut expr = self.unary();
        loop {
            match self.tokens.peek() {
                Some(token) => {
                    match token.r#type {
                        token_type::TokenType::Slash | token_type::TokenType::Star => {
                            let operator = self.tokens.next().unwrap();
                            let right = self.unary();
                            expr = expr::Expr::Binary(Box::new(
                                    expr::Binary {
                                        left: expr,
                                        operator,
                                        right,
                                    }
                                    ));
                        },
                        _ => break,
                    }
                },
                None => break,
            }
        }
        expr
    }

    // BNF: unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> expr::Expr {
        match self.tokens.peek() {
            Some(token) => {
                match token.r#type {
                    token_type::TokenType::Bang | token_type::TokenType::Minus => {
                        let operator = self.tokens.next().unwrap();
                        let right = self.unary();
                        return expr::Expr::Unary(Box::new(
                                expr::Unary {
                                    operator,
                                    right,
                                }
                                ));
                    },
                    _ => {},
                }
            },
            None => {},
        }
        self.primary()
    }

    // BNF: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    // TODO: rewrite
    fn primary(&mut self) -> expr::Expr {
        match self.tokens.peek() {
            Some(token) => {
                match token.r#type {
                    token_type::TokenType::False => expr::Expr::Literal(expr::Literal::Bool(false)),
                    token_type::TokenType::True => expr::Expr::Literal(expr::Literal::Bool(true)),
                    token_type::TokenType::Nil => expr::Expr::Literal(expr::Literal::Nil),
                    token_type::TokenType::Number => expr::Expr::Literal(
                        match token.literal.as_ref().unwrap() {
                            token_type::Literal::Number(x) => expr::Literal::Number(*x),
                            _ => panic!(),
                        }),
                    token_type::TokenType::String => expr::Expr::Literal(
                        match token.literal.as_ref().unwrap() {
                            token_type::Literal::String(x) => expr::Literal::String(x.clone()),
                            _ => panic!(),
                        }),
                    token_type::TokenType::LeftParen => {
                        let expr = self.expression();
                        match self.tokens.peek() {
                            Some(token2) => {
                                match token2.r#type {
                                    token_type::TokenType::RightParen => {
                                        expr::Expr::Grouping(Box::new(
                                                expr::Grouping {
                                                    expression: expr
                                                }
                                                ))
                                    },
                                    _ => panic!("Unexpected token: {}", token2),
                                }
                            },
                            None => panic!("Unexpected token"),
                        }
                    },
                    _ => panic!("Unexpected token: {}", token),
                }
            },
            None => panic!("Unexpected token: None"),
        } 
    }
}
