use crate::lang::KIDL;

#[macro_use]
pub mod macros;

mod helpers;
mod ident;
pub mod schema;
pub mod r#struct;
pub mod r#type;

pub use rowan::ast::AstNode;
pub use rowan::NodeOrToken;
pub use rowan::TextSize;

pub type SyntaxNode = rowan::SyntaxNode<KIDL>;
pub type SyntaxToken = rowan::SyntaxToken<KIDL>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
