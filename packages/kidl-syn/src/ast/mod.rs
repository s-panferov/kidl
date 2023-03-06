use crate::lang::KIDL;

#[macro_use]
pub mod macros;

mod helpers;
pub type SyntaxNode = rowan::SyntaxNode<KIDL>;
pub type SyntaxToken = rowan::SyntaxToken<KIDL>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
