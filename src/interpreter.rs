use crate::expr::{Expr, Literal};
use crate::token_type::TokenType;
use std::{error::Error, fmt};

pub fn interpret(expr: Expr) -> Result<Literal, InterpreterError> {
    match expr {
        Expr::Literal(literal) => {
            Ok(literal)
        }
        Expr::Grouping(grouping) => {
            interpret(grouping.expression)
        }
        Expr::Unary(unary) => {
            let right = interpret(unary.right)?;
            match unary.operator.r#type {
                TokenType::Minus => {
                    match right {
                        Literal::Number(x) => Ok(Literal::Number(-x)),
                        _ => Err(InterpreterError{description:"Can only negate a Number".to_string()})

                    }
                }
                TokenType::Bang => {
                    Ok(Literal::Bool(is_truthy(right)))
                }
                _ => Err(InterpreterError{description:"Unrecognized unary operator".to_string()})
            }
        }
        Expr::Binary(binary) => {
            let left = interpret(binary.left)?;
            let right = interpret(binary.right)?;
            let types = (left, right);
            match binary.operator.r#type {
                TokenType::Minus => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x - y)),
                        _ => Err(InterpreterError{description:"Can only subtract two Numbers".to_string()})
                    }
                }
                TokenType::Slash => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x / y)),
                        _ => Err(InterpreterError{description:"Can only divide two Numbers".to_string()})
                    }
                }
                TokenType::Star => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x * y)),
                        _ => Err(InterpreterError{description:"Can only multiply two Numbers".to_string()})
                    }
                }
                TokenType::Plus => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x + y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::String(x + y.as_str())),
                        _ => Err(InterpreterError{description:"Can only add two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::Greater => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Bool(x > y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::Bool(x > y)),
                        _ => Err(InterpreterError{description:"Can only use greater than operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::Less => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Bool(x < y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::Bool(x < y)),
                        _ => Err(InterpreterError{description:"Can only use less than operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::GreaterEqual => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Bool(x >= y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::Bool(x >= y)),
                        _ => Err(InterpreterError{description:"Can only use greater than or equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::LessEqual => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Bool(x <= y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::Bool(x <= y)),
                        _ => Err(InterpreterError{description:"Can only use less than or equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::BangEqual => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Bool(x != y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::Bool(x != y)),
                        _ => Err(InterpreterError{description:"Can only use bang equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                TokenType::EqualEqual => {
                    match types {
                        (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Bool(x == y)),
                        (Literal::String(x), Literal::String(y)) => Ok(Literal::Bool(x == y)),
                        _ => Err(InterpreterError{description:"Can only use equal equal operator on two Numbers or two Strings".to_string()})
                    }
                }
                _ => Err(InterpreterError{description:"Unrecognized binary operator".to_string()})
            }
        }
    }
}
fn is_truthy(literal: Literal) -> bool {
    match literal {
        Literal::Nil => false,
        Literal::Bool(b) => b,
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
