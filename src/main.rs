use std::{env,fs,process};
use std::io::{stdin, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut lox: Lox = Lox { hadError:false };
    match args.len() {
        0 => Lox::runPrompt(&mut lox),
        1 => Lox::runFile(&mut lox, &args[0]),
        _ => panic!("Usage: rlox [script]")
    }
}

struct Lox {
    hadError: bool,
}

impl Lox {
    fn runFile(&mut self, path: &String) {
        let contents = fs::read_to_string(path).expect(format!("Failed to read file from {}", path).as_str());
        self.run(contents);
        if self.hadError {process::exit(65);}
    }

    fn runPrompt(&mut self) {
        let stdin = stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => Lox::run(self, line),
                Err(_) => break,
            }
        }
    }

    fn run(&mut self, source: String) {
        let scanner: Scanner = Scanner { source };
        let tokens = scanner.scanTokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(mut self, line: usize, message: String) {
        Lox::report(self, line, "".to_string(), message);
    }

    fn report(mut self, line: usize, location: String, message: String) {
        println!("[line {}] Error{}: {}", line, location, message);
        self.hadError = true;
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
