use rowan::ast::AstNode;

use crate::kind::{NodeKind, SyntaxKind, TokenKind};

use super::{
    helpers::{filter_all, find_of, HasName},
    r#type::Type,
    SyntaxNode, SyntaxToken,
};

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Struct(pub SyntaxNode);

ast_node!(Struct, SyntaxKind::Node(NodeKind::Struct));

impl HasName for Struct {}

impl Struct {
    pub fn fields(&self) -> impl Iterator<Item = StructField> {
        filter_all::<StructField>(self.syntax())
    }
}

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StructField(pub SyntaxNode);

ast_node!(StructField, SyntaxKind::Node(NodeKind::StructField));

impl HasName for StructField {}

impl StructField {
    pub fn ty(&self) -> Option<Type> {
        find_of::<Type>(self.syntax())
    }

    pub fn is_optional(&self) -> Option<SyntaxToken> {
        self.0.children_with_tokens().find_map(|t| {
            if t.kind() == SyntaxKind::Token(TokenKind::Question) {
                return t.as_token().cloned();
            } else {
                None
            }
        })
    }
}
