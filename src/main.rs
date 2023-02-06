use std::{cell::RefCell, collections::HashMap, rc::Rc};
mod environment;
mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token_type;

fn main() {
    let mut lox = lox::Lox {
        had_error: false,
        interpreter: interpreter::Interpreter {
            environment: Rc::new(RefCell::new(environment::Environment {
                values: HashMap::new(),
                enclosing: None,
            })),
        },
    };
    lox.main();
}
