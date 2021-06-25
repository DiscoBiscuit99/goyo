# The Grammer

+ Literals
    - numbers
    - strings
    - booleans
+ Unary expressions
    - `-` negates a number
    - `!` negates a logical statement
+ Binary expressions
    - Infix logic operators (`+`, `-`, `*`, `/`) 
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
