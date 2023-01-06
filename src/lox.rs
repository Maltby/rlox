use clap::{App,Arg};
use std::fs;
use std::io;
use std::process;

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

    fn run_file(&self, filepath: &str) {
        let contents = fs::read_to_string(filepath)
            .expect(&format!("Failed to read from given filepath: {:?}", filepath));
        Lox::run(contents);
        if self.had_error {process::exit(65);}
    }

    fn run_prompt(&mut self) {
        let lines = io::stdin().lines();
        for line in lines {
            Lox::run(line.unwrap());
            self.had_error = false;
        }
    }

    fn run(source: String) {
        let mut scanner: Scanner = Scanner {source};
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("token: {token}");
        }
    }

    fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, _where: &str, message: &str) {
        println!("[line {line}] Error{_where}: {message}");
    }
}

struct Scanner {
    source: String
}
impl Scanner {
    fn scan_tokens(&mut self) -> Vec<&str> {
        let mut tokens: Vec<&str> = vec!();
        for line in self.source.lines() {
            tokens.push(line);
        }
        tokens
    }
}
