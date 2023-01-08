mod lox;

fn main() {
    let mut lox = lox::Lox { had_error: false };
    lox.main();
}
