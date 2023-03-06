use crate::kind::{NodeKind, SyntaxKind};

use super::{helpers::HasName, SyntaxNode};

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Type(pub SyntaxNode);

ast_node!(Type, SyntaxKind::Node(NodeKind::Type));

impl HasName for Type {}
