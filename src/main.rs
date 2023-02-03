mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token_type;

fn main() {
    let mut lox = lox::Lox { had_error: false };
    lox.main();
}
