use clap::{App,Arg};
use std::fs;
use std::io;

fn main() {
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
    fn scan_tokens(&mut self) {
        for line in self.source.lines() {
            println!("line: {}", line);
        }
    }
}

fn run(source: String) {
    let mut scanner: Scanner = Scanner {source};
    scanner.scan_tokens();
}
