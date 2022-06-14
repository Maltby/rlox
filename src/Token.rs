use TokenType;
struct Token {
    loxType: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
}

impl Token {
    fn Token(loxType: TokenType, lexeme: string, literal: Object, line: usize) -> Token {
        Token {loxType, lexeme, literal, line}
    }

    // TODO: Should be implemented as trait
    pub fn toString(self) -> String {
        format!("{} {} {}", self.loxType, self.lexeme, self.literal).to_string()
    }
}