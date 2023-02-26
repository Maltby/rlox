use crate::expr;
use crate::token_type;

pub enum Stmt {
    Expr(Expr),
    Print(Print),
    VarDec(VarDec),
    Block(Block),
    If(If),
    While(While),
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
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct While {
    pub condition: expr::Expr,
    pub body: Box<Stmt>,
}
