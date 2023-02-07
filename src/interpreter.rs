use crate::environment;
use crate::expr;
use crate::stmt;
use crate::token_type::TokenType;
use std::{cell::RefCell, collections::HashMap, error::Error, fmt, rc::Rc};

pub struct Interpreter {
    pub environment: Rc<RefCell<environment::Environment>>,
}
impl Interpreter {
    pub fn interpret_stmts(&mut self, stmts: &Vec<stmt::Stmt>) -> Result<(), InterpreterError> {
        for stmt in stmts {
            self.stmt(&stmt)?;
        }
        Ok(())
    }

    pub fn stmt(&mut self, stmt: &stmt::Stmt) -> Result<(), InterpreterError> {
        match stmt {
            stmt::Stmt::Expr(expr_stmt) => {
                self.expr(&expr_stmt.expression)?;
            }
            stmt::Stmt::Print(print_stmt) => self.print_stmt(&print_stmt.expression)?,
            stmt::Stmt::VarDec(var_stmt) => self.var_stmt(var_stmt)?,
            stmt::Stmt::Block(block_stmt) => self.block_stmt(&block_stmt.statements)?,
            stmt::Stmt::If(if_stmt) => self.if_stmt(if_stmt)?,
            stmt::Stmt::While(while_stmt) => self.while_stmt(while_stmt)?,
        };
        Ok(())
    }

    pub fn while_stmt(&mut self, stmt: &stmt::While) -> Result<(), InterpreterError> {
        while Self::is_truthy(&self.expr(&stmt.condition)?) {
            self.stmt(&stmt.body)?;
        }
        Ok(())
    }

    pub fn if_stmt(&mut self, stmt: &stmt::If) -> Result<(), InterpreterError> {
        if Self::is_truthy(&self.expr(&stmt.condition)?) {
            return self.stmt(&stmt.then_branch);
        } else if stmt.else_branch.is_some() {
            return self.stmt(&stmt.else_branch.as_ref().unwrap());
        }
        Ok(())
    }

    pub fn block_stmt(&mut self, stmts: &Vec<stmt::Stmt>) -> Result<(), InterpreterError> {
        let tmp = self.environment.clone();
        self.environment = Rc::new(RefCell::new(environment::Environment {
            enclosing: Some(self.environment.clone()),
            values: HashMap::new(),
        }));
        self.interpret_stmts(stmts)?;
        self.environment = tmp;
        Ok(())
    }

    pub fn var_stmt(&mut self, stmt: &stmt::VarDec) -> Result<(), InterpreterError> {
        let value = match &stmt.expression {
            Some(expr) => Some(self.expr(&expr)?),
            None => None,
        };
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    pub fn print_stmt(&mut self, expr: &expr::Expr) -> Result<(), InterpreterError> {
        let value = self.expr(expr)?;
        println!("{}", value);
        Ok(())
    }

    pub fn expr(&mut self, expr: &expr::Expr) -> Result<expr::Literal, InterpreterError> {
        match expr {
            expr::Expr::Literal(literal) => Ok(literal.clone()),
            expr::Expr::Grouping(grouping) => self.expr(&grouping.expression),
            expr::Expr::Unary(unary) => {
                let right = self.expr(&unary.right)?;
                match unary.operator.r#type {
                    TokenType::Minus => match right {
                        expr::Literal::Number(x) => Ok(expr::Literal::Number(-x)),
                        _ => Err(InterpreterError {
                            description: "Can only negate a Number".to_string(),
                        }),
                    },
                    TokenType::Bang => Ok(expr::Literal::Bool(Self::is_truthy(&right))),
                    _ => Err(InterpreterError {
                        description: "Unrecognized unary operator".to_string(),
                    }),
                }
            }
            expr::Expr::Binary(binary) => {
                let left = self.expr(&binary.left)?;
                let right = self.expr(&binary.right)?;
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
            expr::Expr::Variable(variable) => {
                match self.environment.borrow_mut().get(variable.name.clone()) {
                    Ok(literal) => Ok(literal),
                    Err(e) => Err(InterpreterError {
                        description: e.description,
                    }),
                }
            }
            expr::Expr::Assign(assign) => {
                let value = self.expr(&assign.value)?;
                match self
                    .environment
                    .borrow_mut()
                    .assign(assign.name.clone(), value.clone())
                {
                    Ok(_) => Ok(value),
                    Err(e) => Err(InterpreterError {
                        description: e.description,
                    }),
                }
            }
            expr::Expr::Logical(logical) => {
                let left = self.expr(&logical.left)?;
                match logical.operator.r#type {
                    TokenType::And => {
                        if !Self::is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    TokenType::Or => {
                        if Self::is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    _ => {
                        return Err(InterpreterError {
                            description: format!(
                                "Logical expression created with an unsupported operator: {}",
                                logical.operator.lexeme
                            ),
                        })
                    }
                }
                self.expr(&logical.right)
            }
        }
    }

    fn is_truthy(literal: &expr::Literal) -> bool {
        match literal {
            expr::Literal::Nil => false,
            expr::Literal::Bool(b) => *b,
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
