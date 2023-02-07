use crate::expr;
use crate::stmt;
use crate::token_type;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Print>),
    VarDec(Box<VarDec>),
    Block(Box<Block>),
    If(Box<If>),
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

pub struct If {
    pub condition: expr::Expr,
    pub then_branch: stmt::Stmt,
    pub else_branch: Option<stmt::Stmt>,
}
