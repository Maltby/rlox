use clap::{App,Arg};
use std::fs;
use std::io;

pub struct Lox {
    pub had_error: bool
}
impl Lox {
    pub fn main(&self) {
        let args = App::new("rlox")
            .arg(Arg::with_name("filepath")
                 .takes_value(true))
            .get_matches();

        match args.value_of("filepath") {
            Some(filepath) => {
                run_file(filepath);
            },
            None => {
                run_prompt();
            }
        }
    }
}

fn run_file(filepath: &str) {
    let contents = fs::read_to_string(filepath)
        .expect(&format!("Failed to read from given filepath: {:?}", filepath));
    run(contents);
}

fn run_prompt() {
    let lines = io::stdin().lines();
    for line in lines {
        run(line.unwrap());
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

fn run(source: String) {
    let mut scanner: Scanner = Scanner {source};
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("token: {token}");
    }

}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, _where: &str, message: &str) {
    println!("[line {line}] Error{_where}: {message}");
}
