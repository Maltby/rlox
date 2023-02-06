use crate::expr;
use crate::token_type;
use std::collections::HashMap;
use std::{cell::RefCell, error::Error, fmt, rc::Rc};

#[derive(Debug)]
pub struct RuntimeError {
    pub description: String,
}
impl Error for RuntimeError {}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, expr::Literal>,
}
impl Environment {
    pub fn define(&mut self, name: String, value: Option<expr::Literal>) {
        match value {
            Some(value) => self.values.insert(name, value),
            None => self.values.insert(name, expr::Literal::Nil),
        };
    }

    pub fn assign(
        &mut self,
        name: token_type::Token,
        value: expr::Literal,
    ) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }
        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        Err(RuntimeError {
            description: format!("Undefined variable {}", name.lexeme),
        })
    }

    pub fn get(&self, name: token_type::Token) -> Result<expr::Literal, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().get(name);
        }
        Err(RuntimeError {
            description: format!("Undefined variable {}", name.lexeme),
        })
    }
}
