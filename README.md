Implementing Lox in Rust as I read through *Crafting Interpreters*

Our starting syntactic grammar:
```
expression -> literal | unary | binary | grouping;
literal    -> NUMBER | STRING | "true" | "false" | "nil";
grouping   -> "(" expression ")";
unary      -> ( "-" | "!" ) expression;
binary     -> expression operator expression;
operator   -> "==" | "!=" | "<" | "<=" | ">" | ">=" |
			  "+" | "-" | "*" | "/";
```
