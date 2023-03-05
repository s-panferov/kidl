```rust
struct Struct { }
```

```
Node(Root)(
    Node(Error)(
        Token(Ident)["struct"],
    ),
    Token(Space),
    Node(Error)(
        Token(Ident)["Struct"],
    ),
    Token(Space),
    Node(Error)(
        Token(CurlyOpen),
    ),
    Token(Space),
    Node(Error)(
        Token(CurlyClose),
    ),
)
```
