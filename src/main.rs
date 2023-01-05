mod lox;

fn main() {
    let lox = lox::Lox { had_error: false };
    lox.main();
}
