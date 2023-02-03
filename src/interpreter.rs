use crate::expr;
use crate::stmt;
use crate::token_type::TokenType;
use std::{error::Error, fmt};

pub fn interpret_stmts(stmts: Vec<stmt::Stmt>) -> Result<(), InterpreterError> {
    for stmt in stmts {
        match stmt {
            stmt::Stmt::Expr(expr_stmt) => {
                interpret_expr(expr_stmt.expression)?;
            }
            stmt::Stmt::Print(print_stmt) => interpret_print(print_stmt.expression)?,
        };
    }
    Ok(())
}

pub fn interpret_print(expr: expr::Expr) -> Result<(), InterpreterError> {
    let value = interpret_expr(expr)?;
    println!("{}", value);
    Ok(())
}

pub fn interpret_expr(expr: expr::Expr) -> Result<expr::Literal, InterpreterError> {
    match expr {
        expr::Expr::Literal(literal) => Ok(literal),
        expr::Expr::Grouping(grouping) => interpret_expr(grouping.expression),
        expr::Expr::Unary(unary) => {
            let right = interpret_expr(unary.right)?;
            match unary.operator.r#type {
                TokenType::Minus => match right {
                    expr::Literal::Number(x) => Ok(expr::Literal::Number(-x)),
                    _ => Err(InterpreterError {
                        description: "Can only negate a Number".to_string(),
                    }),
                },
                TokenType::Bang => Ok(expr::Literal::Bool(is_truthy(right))),
                _ => Err(InterpreterError {
                    description: "Unrecognized unary operator".to_string(),
                }),
            }
        }
        expr::Expr::Binary(binary) => {
            let left = interpret_expr(binary.left)?;
            let right = interpret_expr(binary.right)?;
            let types = (left, right);
            match binary.operator.r#type {
                TokenType::Minus => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Number(x - y)),
                        _ => Err(InterpreterError{description:"Can only subtract two Numbers".to_string()})
                    }
                }
                TokenType::Slash => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Number(x / y)),
                        _ => Err(InterpreterError{description:"Can only divide two Numbers".to_string()})
                    }
                }
                TokenType::Star => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Number(x * y)),
                        _ => Err(InterpreterError{description:"Can only multiply two Numbers".to_string()})
                    }
                }
                TokenType::Plus => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Number(x + y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::String(x + y.as_str())),
                        _ => Err(InterpreterError{description:"Can only add two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::Greater => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Bool(x > y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::Bool(x > y)),
                        _ => Err(InterpreterError{description:"Can only use greater than operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::Less => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Bool(x < y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::Bool(x < y)),
                        _ => Err(InterpreterError{description:"Can only use less than operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::GreaterEqual => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Bool(x >= y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::Bool(x >= y)),
                        _ => Err(InterpreterError{description:"Can only use greater than or equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::LessEqual => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Bool(x <= y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::Bool(x <= y)),
                        _ => Err(InterpreterError{description:"Can only use less than or equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::BangEqual => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Bool(x != y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::Bool(x != y)),
                        _ => Err(InterpreterError{description:"Can only use bang equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::EqualEqual => {
                    match types {
                        (expr::Literal::Number(x), expr::Literal::Number(y)) => Ok(expr::Literal::Bool(x == y)),
                        (expr::Literal::String(x), expr::Literal::String(y)) => Ok(expr::Literal::Bool(x == y)),
                        _ => Err(InterpreterError{description:"Can only use equal equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                _ => Err(InterpreterError{description:"Unrecognized binary operator".to_string()})
            }
        }
    }
}
fn is_truthy(literal: expr::Literal) -> bool {
    match literal {
        expr::Literal::Nil => false,
        expr::Literal::Bool(b) => b,
        _ => true,
    }
}

#[derive(Debug)]
pub struct InterpreterError {
    description: String,
}
impl Error for InterpreterError {}
impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InterpreterError: {}", self.description)
    }
}
