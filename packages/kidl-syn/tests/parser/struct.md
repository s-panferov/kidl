```rust
struct Struct {
    field: Test,
}  
```

``` --draft
Node(Root)(
    Node(Struct)(
        Token(Ident)["struct"],
        Token(Space),
        Token(Ident)["Struct"],
        Token(Space),
        Token(CurlyOpen),
        Token(NewLine),
        Token(Space),
        Node(StructField)(
            Token(Ident)["field"],
            Token(Colon),
            Token(Space),
            Node(Type)(
                Token(Ident)["Test"],
            ),
            Token(Comma),
        ),
        Token(NewLine),
        Token(CurlyClose),
        Token(Space),
    ),
) [
    SyntaxError(
        "Expected CurlyOpen, found Ident",
        Byte(7)..Byte(7),
    ),
]
```
