use std::{env,fs,io};
use std::io::{BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => runPrompt(),
        1 => runFile(&args[0]),
        _ => panic!("Usage: rlox [script]")
    }
}

fn runFile(path: &String) {
    let contents = fs::read_to_string(path).expect(format!("Failed to read file from {}", path).as_str());
}

fn runPrompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => run(line),
            Err(_) => break,
        }
    }
}

fn run(source: String) {
    let scanner: Scanner = Scanner {source};
    let tokens = scanner.scanTokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

pub struct Scanner {
    source: String,
}
impl Scanner {
    fn scanTokens(&self) -> Vec<Token> {
        vec!()
    }
}

#[derive(Debug)]
enum Token {}