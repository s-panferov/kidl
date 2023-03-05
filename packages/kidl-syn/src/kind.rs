#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    Token(TokenKind),
    Node(NodeKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum TokenKind {
    CurlyOpen = 0,
    CurlyClose,
    SquareOpen,
    SquareClose,
    AngleOpen,
    AngleClose,
    ParenOpen,
    ParenClose,
    Comment,
    Comma,
    Dot,
    Hash,
    Question,
    Eq,
    Colon,
    Semicolon,
    Space,
    NewLine,
    Ident,
    String,
    Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum NodeKind {
    Root = 1000,
    Error,
    Struct,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        match kind {
            SyntaxKind::Token(t) => rowan::SyntaxKind(t as u16),
            SyntaxKind::Node(n) => rowan::SyntaxKind(n as u16),
        }
    }
}

impl From<NodeKind> for rowan::SyntaxKind {
    fn from(kind: NodeKind) -> Self {
        rowan::SyntaxKind(kind as u16)
    }
}

impl From<TokenKind> for rowan::SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        rowan::SyntaxKind(kind as u16)
    }
}
