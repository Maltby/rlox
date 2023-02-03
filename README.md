Implementing Lox in Rust as I read through *Crafting Interpreters*

Backus-Naur Form:
```
program    -> statement* EOF;
statement  -> exprStmt | printStmt;
exprStmt   -> expression ";";
printStmt  -> "print" expression ";";
expression -> literal | unary | binary | grouping;
literal    -> NUMBER | STRING | "true" | "false" | "nil";
grouping   -> "(" expression ")";
unary      -> ( "-" | "!" ) expression;
binary     -> expression operator expression;
operator   -> "==" | "!=" | "<" | "<=" | ">" | ">=" |
              "+" | "-" | "*" | "/";
```
