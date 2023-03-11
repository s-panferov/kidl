use crate::{helpers::ByteOffset, kind::TokenKind, lexer::Token};

use super::error::SyntaxError;

pub trait ExpectPredicate {
    fn matches(&self, token: &Token) -> bool;
    fn error(&self, failed: &Token, offset: ByteOffset) -> SyntaxError;
}

pub struct TokenPredicate(TokenKind);

impl ExpectPredicate for TokenPredicate {
    fn matches(&self, token: &Token) -> bool {
        token.kind == self.0
    }

    fn error(&self, failed: &Token, offset: ByteOffset) -> SyntaxError {
        SyntaxError::new_at_offset(
            format!("Expected {:?}, found {:?}", self.0, failed.kind),
            offset,
        )
    }
}

pub struct IdentPredicate(&'static str);

impl ExpectPredicate for IdentPredicate {
    fn matches(&self, token: &Token) -> bool {
        token.kind == TokenKind::Ident && token.slice == self.0
    }

    fn error(&self, failed: &Token, offset: ByteOffset) -> SyntaxError {
        SyntaxError::new_at_offset(
            format!(
                "Expected identifier \"{:?}\", found {:?}",
                self.0, failed.kind
            ),
            offset,
        )
    }
}

pub struct FnPredicate<F, E>(F, E)
where
    F: Fn(&Token) -> bool,
    E: Fn(&Token, ByteOffset) -> SyntaxError;

impl<F, E> ExpectPredicate for FnPredicate<F, E>
where
    F: Fn(&Token) -> bool,
    E: Fn(&Token, ByteOffset) -> SyntaxError,
{
    fn matches(&self, token: &Token) -> bool {
        self.0(token)
    }

    fn error(&self, failed: &Token, offset: ByteOffset) -> SyntaxError {
        self.1(failed, offset)
    }
}

pub fn kind(kind: TokenKind) -> TokenPredicate {
    TokenPredicate(kind)
}

pub fn ident(text: &'static str) -> IdentPredicate {
    IdentPredicate(text)
}

pub const TRIVIA: [TokenKind; 2] = [TokenKind::Space, TokenKind::Comment];
pub const TRIVIA_NL: [TokenKind; 3] = [TokenKind::Space, TokenKind::Comment, TokenKind::NewLine];

pub fn with_trivia(inner: impl Fn(&Token) -> bool) -> impl Fn(&Token) -> bool {
    move |token| match token.kind {
        trivia!() => true,
        _ => inner(token),
    }
}

pub fn with_trivia_nl(inner: impl Fn(&Token) -> bool) -> impl Fn(&Token) -> bool {
    move |token| match token.kind {
        trivia_with_newline!() => true,
        _ => inner(token),
    }
}
