use rowan::{GreenNode, GreenToken, Language, NodeOrToken};

use crate::{
    kind::{SyntaxKind, TokenKind},
    lang::KIDL,
};

pub struct DebugNodePrinter(pub GreenNode);

impl std::fmt::Debug for DebugNodePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.0.kind();
        let kind: SyntaxKind = KIDL::kind_from_raw(kind);

        let mut printer = f.debug_tuple(&format!("{:?}", kind));
        for node in self.0.children() {
            match node {
                NodeOrToken::Node(node) => printer.field(&DebugNodePrinter(node.to_owned())),
                NodeOrToken::Token(node) => printer.field(&DebugTokenPrinter(node.to_owned())),
            };
        }

        printer.finish()
    }
}

pub struct DebugTokenPrinter(pub GreenToken);

impl std::fmt::Debug for DebugTokenPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.0.kind();
        let kind: SyntaxKind = KIDL::kind_from_raw(kind);

        match kind {
            SyntaxKind::Token(
                TokenKind::Ident | TokenKind::Number | TokenKind::Comment | TokenKind::String,
            ) => write!(f, "{:?}[{:?}]", kind, self.0.text()),
            _ => {
                write!(f, "{:?}", kind)
            }
        }
    }
}
