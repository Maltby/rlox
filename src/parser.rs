use crate::token_type;
use crate::expr;

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<token_type::Token>>,
}
impl Parser {
    // BNF: expression -> equality ;
    fn expression(&mut self) -> expr::Expr {
        self.equality() 
    }

    // BNF: equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> expr::Expr {
        let mut expr = self.comparison();
        while let Some(token) = self.tokens.peek() {
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
        }
        expr
    }

    // BNF: comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> expr::Expr {
        let mut expr = self.term();
        while let Some(token) = self.tokens.peek() {
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
        }
        expr
    }

    fn term(&mut self) -> expr::Expr {
        let mut expr = self.factor();
        while let Some(token) = self.tokens.peek() {
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
        }
        expr
    }

    fn factor(&mut self) -> expr::Expr {
        let mut expr = self.unary();
        while let Some(token) = self.tokens.peek() {
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
        }
        expr
    }

    // BNF: unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> expr::Expr {
        if let Some(token) = self.tokens.peek() {
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
        }
        self.primary()
    }

    // BNF: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> expr::Expr {
        let token = match self.tokens.next() {
            Some(token) => token,
            None => panic!("Expected a token")
        };
        match token.r#type {
            token_type::TokenType::False => expr::Expr::Literal(expr::Literal::Bool(false)),
            token_type::TokenType::True => expr::Expr::Literal(expr::Literal::Bool(true)),
            token_type::TokenType::Nil => expr::Expr::Literal(expr::Literal::Nil),
            token_type::TokenType::Number => expr::Expr::Literal(
                match token.literal.as_ref().unwrap() {
                    token_type::Literal::Number(x) => expr::Literal::Number(*x),
                    _ => panic!("Number token did not contain a Number"),
                }),
            token_type::TokenType::String => expr::Expr::Literal(
                match token.literal.as_ref().unwrap() {
                    token_type::Literal::String(x) => expr::Literal::String(x.clone()),
                    _ => panic!("String token did not contain a String"),
                }),
            token_type::TokenType::LeftParen => {
                let expr = self.expression();
                if self.tokens.peek().unwrap().r#type == token_type::TokenType::RightParen {
                    self.tokens.next(); // consume RightParen
                    expr::Expr::Grouping(Box::new(
                            expr::Grouping {
                                expression: expr
                            }))
                } else {
                    panic!("Grouping did not end in right paren");
                }
            },
            _ => panic!("Unexpected token: {}", token),
        }
    }
}
