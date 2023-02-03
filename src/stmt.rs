use crate::expr;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Print>),
}

pub struct Expr {
    pub expression: expr::Expr,
}

pub struct Print {
    pub expression: expr::Expr,
}
