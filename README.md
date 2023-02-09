#### Implementing Lox in Rust as I read through *Crafting Interpreters*
The tree-walk-interpreter version of Lox is implemented in Java within *Crafting Interpreters*, this is my port to Rust.

#### Build:
`cargo build`

#### Run:
`cargo run <optional filepath>`
*Run without filepath to enter REPL*

#### Example:
`cargo run ./examples/showcase.lox`

#### Backus-Naur Form:
```
program     -> declaration* EOF ;
declaration -> varDecl | statement ;
varDecl     -> "var" IDENTIFIER ( "=" expression )? ";" ;
statement   -> exprStmt | printStmt | block | ifStmt ;
ifStmt      -> "if" "(" expression ")" statement 
               ( "else" statement )? ;
expression  -> assignment ;
assignment  -> IDENTIFIER "=" assignment | logicOr ;
logicOr     -> logicAnd ( "or" logicAnd )* ;
logicAnd    -> equality ( "and" equality )* ;
printStmt   -> "print" expression ";" ;
block       -> "{" declaration* "}" ;
equality    -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison  -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term        -> factor ( ( "-" | "+" ) factor )* ;
factor      -> unary ( ( "/" | "*" ) unary )* ;
unary       -> ( "!" | "-" ) unary | call ;
call        -> primary ( "(" arguments? ")" )* ;
arguments   -> expression ( "," expression )* ;
primary     -> NUMBER | STRING | "true" | "false" | "nil" |
               "(" expression ")" | IDENTIFIER ;
```
