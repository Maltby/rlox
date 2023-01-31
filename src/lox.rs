use clap::{App,Arg};
use std::fs;
use std::io;
use std::process;
use crate::scanner::Scanner;
use crate::parser::*;
use crate::interpreter::interpret;

pub struct Lox {
    pub had_error: bool
}
impl Lox {
    pub fn main(&mut self) {
        let args = App::new("rlox")
            .arg(Arg::with_name("filepath")
                 .takes_value(true))
            .get_matches();

        match args.value_of("filepath") {
            Some(filepath) => {
                self.run_file(filepath);
            },
            None => {
                self.run_prompt();
            }
        }
    }

    fn run_file(&mut self, filepath: &str) {
        let contents = fs::read_to_string(filepath)
            .expect(&format!("Failed to read from given filepath: {:?}", filepath));
        Lox::run(self, contents);
        if self.had_error {
            process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
        let lines = io::stdin().lines();
        for line in lines {
            Lox::run(self, line.unwrap());
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let tokens = match Scanner::scan_tokens(source) {
            Ok(tokens) => tokens,
            Err(errors) => {
                self.had_error = true;
                for error in errors {
                    Self::report(error, "");
                }
                return;
            }
        };
        match Parser::parse(tokens) {
            Ok(expr) => {
                match interpret(expr) {
                    Ok(value) => println!("{}", value),
                    Err(e) => println!("{}", e)
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    fn report(error: Error, _where: &str) {
        println!("[line {0}] Error{1}: {2}", error.line, _where, error.message);
    }
}

pub struct Error {
    pub line: usize,
    pub message: String,
}
