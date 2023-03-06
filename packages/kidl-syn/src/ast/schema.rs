use crate::kind::{NodeKind, SyntaxKind};

use super::{r#struct::Struct, SyntaxNode};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
#[repr(transparent)]
pub struct Schema(SyntaxNode);

ast_node!(Schema, SyntaxKind::Node(NodeKind::Root));

pub enum Declaration {
    Struct(Struct),
}

impl Schema {
    pub fn declarations(&self) -> impl Iterator<Item = Declaration> {
        self.0.children().filter_map(|c| match c.kind() {
            SyntaxKind::Node(NodeKind::Struct) => Some(Declaration::Struct(Struct(c))),
            _ => None,
        })
    }
}
