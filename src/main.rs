mod lox;
mod scanner;
mod token_type;
mod expr;

fn main() {
    let e = expr::Expr::Binary(Box::new(expr::Binary{
        left: expr::Expr::Unary(Box::new(expr::Unary{
            operator: token_type::Token{
                r#type: token_type::TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 0,
            },
            right: expr::Expr::Literal(expr::Literal::Number(123.)),        
        })),
        operator: token_type::Token {
            r#type: token_type::TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 0,
        },
        right: expr::Expr::Grouping(Box::new(expr::Grouping{
            expression: expr::Expr::Literal(expr::Literal::Number(45.67)),
        })),
    }));
    println!("{}", e);
    // let mut lox = lox::Lox { had_error: false };
    // lox.main();
}
