macro_rules! ast_node {
    ($ast:ident, $kind:expr) => {
        impl rowan::ast::AstNode for $ast {
            type Language = crate::lang::KIDL;

            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
            where
                Self: Sized,
            {
                kind == $kind
            }

            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                if Self::can_cast(node.kind()) {
                    Some($ast(node))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
                &self.0
            }
        }
    };
}

macro_rules! keywords {
    () => {
        "struct"
    };
}
