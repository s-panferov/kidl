```js
use webidl::AbortSignal;
use "https://hg.mozilla.org/mozilla-central/raw-file/tip/dom/webidl/AbortSignal.webidl"::AbortSignal;

use
use webidl::::
```

```rust --draft
Node(Root)(
    Node(Use)(
        Token(Ident)["use"],
        Token(Space),
        Node(Path)(
            Token(Ident)["webidl"],
            Token(Colon),
            Token(Colon),
            Token(Ident)["AbortSignal"],
        ),
        Token(Semicolon),
    ),
    Node(Use)(
        Token(NewLine),
        Token(Ident)["use"],
        Token(Space),
        Node(Path)(
            Token(String)["\"https://hg.mozilla.org/mozilla-central/raw-file/tip/dom/webidl/AbortSignal.webidl\""],
            Token(Colon),
            Token(Colon),
            Token(Ident)["AbortSignal"],
        ),
        Token(Semicolon),
    ),
    Node(Use)(
        Token(NewLine),
        Token(NewLine),
        Token(Ident)["use"],
        Token(NewLine),
    ),
    Node(Use)(
        Token(Ident)["use"],
        Token(Space),
        Node(Path)(
            Token(Ident)["webidl"],
            Token(Colon),
            Token(Colon),
        ),
    ),
) [
    SyntaxError(
        "Expected valid path, got Some(Token { kind: Ident, slice: \"use\" })",
        Byte(132)..Byte(132),
    ),
    SyntaxError(
        "Unexpected Token { kind: Colon, slice: \":\" }",
        Byte(144)..Byte(144),
    ),
    SyntaxError(
        "Unexpected Token { kind: Colon, slice: \":\" }",
        Byte(144)..Byte(144),
    ),
]
```
