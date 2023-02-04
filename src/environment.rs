use crate::expr;
use crate::token_type;
use std::collections::HashMap;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct RuntimeError {
    description: String,
}
impl Error for RuntimeError {}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

pub struct Environment {
    values: HashMap<String, Option<expr::Literal>>,
}
impl Environment {
    pub fn define(&mut self, name: String, value: Option<expr::Literal>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: token_type::Token) -> Result<&Option<expr::Literal>, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap());
        }
        Err(RuntimeError {
            description: format!("Undefined variable {}", name.lexeme),
        })
    }
}
