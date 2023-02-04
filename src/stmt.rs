use crate::expr;
use crate::token_type;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Print>),
    Var(Box<Var>),
}

pub struct Expr {
    pub expression: expr::Expr,
}

pub struct Print {
    pub expression: expr::Expr,
}

pub struct Var {
    pub name: token_type::Token,
    pub expression: Option<expr::Expr>,
}
