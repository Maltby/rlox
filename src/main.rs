mod lox;
mod token_type;

fn main() {
    let mut lox = lox::Lox { had_error: false };
    lox.main();
}
