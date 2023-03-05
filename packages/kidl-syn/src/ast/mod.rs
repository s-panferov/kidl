use crate::lang::KIDL;

pub type SyntaxNode = rowan::SyntaxNode<KIDL>;
pub type SyntaxToken = rowan::SyntaxToken<KIDL>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
