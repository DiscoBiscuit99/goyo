# The Grammer

+ Literals
    - numbers
    - strings
    - booleans
    - `nil`
+ Unary expressions
    - `-` negates a number
    - `!` negates a logical statement
+ Binary expressions
    - Infix arithmetic operators (`+`, `-`, `*`, `/`) 
    - Logic operators (`=`, `/=`, `<`, `<=`, `>`, `>=`)

```
expression -> literal | grouping | unary | binary ; 

literal    -> NUMBER | STRING | "true" | "false" | "nil" ;
grouping   -> "(" expression ")" ;
unary      -> ("-" | "!") expression ;
binary     -> expression operator expression ;
operator   -> "=" | "/=" | "<" | "<=" | ">" | ">=" 
            | "+" | "-" | "*" | "/" ;
```
