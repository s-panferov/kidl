```rust
struct Struct {
    a: Test,
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
            Token(Ident)["a"],
            Token(Colon),
            Token(Space),
            Node(Type)(
                Token(Ident)["Test"],
            ),
            Token(Comma),
        ),
        Token(NewLine),
        Token(CurlyClose),
    ),
)
```
