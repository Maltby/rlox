Implementing Lox in Rust as I read through *Crafting Interpreters*

Backus-Naur Form:
```
program     -> declaration* EOF ;
declaration -> varDecl | statement ;
varDecl     -> "var" IDENTIFIER ( "=" expression )? ";" ;
statement   -> exprStmt | printStmt | block ;
exprStmt    -> expression ";" ;
expression  -> assignment ;
assignment  -> IDENTIFIER "=" assignment | equality ;
printStmt   -> "print" expression ";" ;
block       -> "{" declaration* "}" ;
equality    -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison  -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term        -> factor ( ( "-" | "+" ) factor )* ;
factor      -> unary ( ( "/" | "*" ) unary )* ;
unary       -> ( "!" | "-" ) unary | primary ;
primary     -> NUMBER | STRING | "true" | "false" | "nil" |
               "(" expression ")" | IDENTIFIER ;
```
