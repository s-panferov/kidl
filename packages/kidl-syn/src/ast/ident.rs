use crate::kind::TokenKind;

use super::{helpers::AstToken, SyntaxToken};

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ident(pub SyntaxToken);

impl Ident {
    pub fn text(&self) -> &str {
        self.0.text()
    }
}

impl AstToken for Ident {
    const KIND: TokenKind = TokenKind::Ident;
    fn new(token: SyntaxToken) -> Self {
        Self(token)
    }
}
