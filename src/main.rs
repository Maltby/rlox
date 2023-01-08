mod lox;
mod scanner;
mod token_type;

fn main() {
    let mut lox = lox::Lox { had_error: false };
    lox.main();
}
