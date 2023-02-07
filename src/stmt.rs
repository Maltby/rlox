use crate::expr;
use crate::token_type;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Print>),
    VarDec(Box<VarDec>),
    Block(Box<Block>),
    If(Box<If>),
    While(Box<While>),
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
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

pub struct While {
    pub condition: expr::Expr,
    pub body: Stmt,
}
