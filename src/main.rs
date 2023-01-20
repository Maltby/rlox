mod lox;
mod scanner;
mod token_type;
mod expr;
mod parser;

fn main() {
    let mut lox = lox::Lox { had_error: false };
    lox.main();
}
