use crate::token_type::Token;
use std::fmt;

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary(x)   => {write!(f, "{}", *x)},
            Expr::Grouping(x) => {write!(f, "{}", *x)},
            Expr::Literal(x)  => {write!(f, "{}", x)},
            Expr::Unary(x)    => {write!(f, "{}", *x)},
        }
    }
}

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

pub struct Grouping {
    pub expression: Expr,
}
impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}
impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(x) => {write!(f, "{}", x)},
            Literal::String(x) => {write!(f, "{}", x)},
            Literal::Bool(x)   => {write!(f, "{}", x)},
            Literal::Nil       => {write!(f, "Nil")},
        }
    }
}
