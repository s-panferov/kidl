```rust
// Comment
interface Test {
  value: 10[]
}
```

```
[
    "Token { kind: Comment, slice: \"// Comment\\n\" }",
    "Token { kind: Ident, slice: \"interface\" }",
    "Token { kind: Space, slice: \" \" }",
    "Token { kind: Ident, slice: \"Test\" }",
    "Token { kind: Space, slice: \" \" }",
    "Token { kind: CurlyOpen, slice: \"{\" }",
    "Token { kind: NewLine, slice: \"\\n\" }",
    "Token { kind: Space, slice: \"  \" }",
    "Token { kind: Ident, slice: \"value\" }",
    "Token { kind: Colon, slice: \":\" }",
    "Token { kind: Space, slice: \" \" }",
    "Token { kind: Number, slice: \"10\" }",
    "Token { kind: SquareOpen, slice: \"[\" }",
    "Token { kind: SquareClose, slice: \"]\" }",
    "Token { kind: NewLine, slice: \"\\n\" }",
    "Token { kind: CurlyClose, slice: \"}\" }",
]
```
