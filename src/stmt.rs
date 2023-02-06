use crate::expr;
use crate::token_type;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Print>),
    VarDec(Box<VarDec>),
    Block(Box<Block>),
}

pub struct Expr {
    pub expression: expr::Expr,
}

pub struct Print {
    pub expression: expr::Expr,
}

pub struct VarDec {
    pub name: token_type::Token,
    pub expression: Option<expr::Expr>,
}

pub struct Block {
    pub statements: Vec<Stmt>,
}
