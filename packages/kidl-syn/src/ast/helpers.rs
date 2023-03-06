use rowan::{ast::AstNode, NodeOrToken};

use crate::{
    kind::{SyntaxKind, TokenKind},
    lang::KIDL,
};

use super::{ident::Ident, SyntaxNode, SyntaxToken};

pub trait AstToken {
    const KIND: TokenKind;
    fn new(token: SyntaxToken) -> Self;
}

pub fn filter_all<T: AstNode<Language = KIDL>>(node: &SyntaxNode) -> impl Iterator<Item = T> {
    node.children().filter_map(|c| T::cast(c))
}

pub fn find_of<T: AstNode<Language = KIDL>>(node: &SyntaxNode) -> Option<T> {
    node.children().find_map(|c| T::cast(c))
}

pub fn first_token_matching(
    node: &SyntaxNode,
    predicate: impl Fn(&SyntaxToken) -> bool,
) -> Option<SyntaxToken> {
    node.children_with_tokens().find_map(|t| match t {
        NodeOrToken::Node(_) => None,
        NodeOrToken::Token(t) => {
            if predicate(&t) {
                Some(t)
            } else {
                None
            }
        }
    })
}

pub fn first_token_of_kind(node: &SyntaxNode, kind: TokenKind) -> Option<SyntaxToken> {
    first_token_matching(node, |t| t.kind() == SyntaxKind::Token(kind))
}

pub fn first_token_of<T: AstToken>(node: &SyntaxNode) -> Option<T> {
    node.children_with_tokens().find_map(|t| match t.kind() {
        kind if kind == SyntaxKind::Token(T::KIND) => Some(T::new(t.into_token().unwrap())),
        _ => None,
    })
}

pub trait HasName: AstNode<Language = KIDL> {
    fn name(&self) -> Option<Ident> {
        self.syntax()
            .children_with_tokens()
            .find_map(|t| match t.kind() {
                SyntaxKind::Token(TokenKind::Ident) => {
                    let token = t.as_token().unwrap();
                    match token.text() {
                        keywords!() => None,
                        _ => Some(Ident(token.clone())),
                    }
                }
                _ => None,
            })
    }
}
