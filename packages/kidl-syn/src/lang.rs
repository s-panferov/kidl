use crate::kind::{NodeKind, SyntaxKind, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KIDL {}

impl rowan::Language for KIDL {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        if raw.0 >= 1000 {
            SyntaxKind::Node(unsafe { std::mem::transmute::<u16, NodeKind>(raw.0) })
        } else {
            SyntaxKind::Token(unsafe { std::mem::transmute::<u16, TokenKind>(raw.0) })
        }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
