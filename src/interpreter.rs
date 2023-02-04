use crate::environment;
use crate::expr;
use crate::stmt;
use crate::token_type::TokenType;
use std::{error::Error, fmt};

struct Interpreter {
    environment: environment::Environment,
}
impl Interpreter {
    pub fn interpret_stmts(&mut self, stmts: Vec<stmt::Stmt>) -> Result<(), InterpreterError> {
        for stmt in stmts {
            match stmt {
                stmt::Stmt::Expr(expr_stmt) => {
                    self.expr(expr_stmt.expression)?;
                }
                stmt::Stmt::Print(print_stmt) => self.print_stmt(print_stmt.expression)?,
                stmt::Stmt::Var(var_stmt) => self.var_stmt(*var_stmt)?,
            };
        }
        Ok(())
    }

    pub fn var_stmt(&mut self, stmt: stmt::Var) -> Result<(), InterpreterError> {
        let value = match stmt.expression {
            Some(expr) => Some(self.expr(expr)?),
            None => None,
        };
        self.environment.define(stmt.name.lexeme, value);
        Ok(())
    }

    pub fn print_stmt(&mut self, expr: expr::Expr) -> Result<(), InterpreterError> {
        let value = self.expr(expr)?;
        println!("{}", value);
        Ok(())
    }

    pub fn expr(&mut self, expr: expr::Expr) -> Result<expr::Literal, InterpreterError> {
        match expr {
            expr::Expr::Literal(literal) => Ok(literal),
            expr::Expr::Grouping(grouping) => self.expr(grouping.expression),
            expr::Expr::Unary(unary) => {
                let right = self.expr(unary.right)?;
                match unary.operator.r#type {
                    TokenType::Minus => match right {
                        expr::Literal::Number(x) => Ok(expr::Literal::Number(-x)),
                        _ => Err(InterpreterError {
                            description: "Can only negate a Number".to_string(),
                        }),
                    },
                    TokenType::Bang => Ok(expr::Literal::Bool(Self::is_truthy(right))),
                    _ => Err(InterpreterError {
                        description: "Unrecognized unary operator".to_string(),
                    }),
                }
            }
            expr::Expr::Binary(binary) => {
                let left = self.expr(binary.left)?;
                let right = self.expr(binary.right)?;
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
            expr::Expr::Variable(variable) => match self.environment.get(variable.name) {
                Ok(maybe_literal) => match maybe_literal {
                    Some(x) => Ok(*x),
                    None => Ok(expr::Literal::Nil),
                },
                Err(_) => Err(InterpreterError {
                    description: format!("Unrecognized variable: {}", variable.name),
                }),
            },
        }
    }

    fn is_truthy(literal: expr::Literal) -> bool {
        match literal {
            expr::Literal::Nil => false,
            expr::Literal::Bool(b) => b,
            _ => true,
        }
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
