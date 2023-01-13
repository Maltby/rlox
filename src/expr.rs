use crate::token_type::Token;

enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
}

struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

struct Grouping {
    expression: Expr
}

struct Unary {
    operator: Token,
    right: Expr
}

enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
