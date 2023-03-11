use crate::{kind::TokenKind, lexer::Token};

const KEYWORDS: [&'static str; 1] = ["use"];
pub fn is_keyword(token: &Token) -> bool {
    token.kind == TokenKind::Ident && KEYWORDS.contains(&token.slice.as_ref())
}
