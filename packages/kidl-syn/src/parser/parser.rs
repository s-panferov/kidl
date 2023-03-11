use std::{iter::Peekable, marker::PhantomData};

use rowan::{GreenNode, GreenNodeBuilder};

use crate::{
    helpers::ByteOffset,
    kind::{NodeKind, TokenKind},
    lexer::{tokenize, Token},
    source::StrSource,
};

pub use rowan::NodeCache;

use super::utility::{combinators::ExpectPredicate, error::SyntaxError};

pub trait TokenIter<'t>: Iterator<Item = Token<'t>> {}
impl<'t, T> TokenIter<'t> for T where T: Iterator<Item = Token<'t>> {}

pub struct Parser<'c, 't, T: TokenIter<'t>> {
    pub(crate) builder: GreenNodeBuilder<'c>,
    pub(crate) tokens: Peekable<T>,
    pub(crate) errors: Vec<SyntaxError>,
    offset: ByteOffset,
    #[cfg(debug_assertions)]
    parsed: String,
    _t: PhantomData<&'t ()>,
}

impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub fn consume(&mut self, token: Token<'t>) {
        #[cfg(debug_assertions)]
        self.parsed.push_str(&token.slice);
        self.offset += ByteOffset(token.slice.len());
        self.builder.token(token.kind.into(), &token.slice)
    }

    pub fn consume_next(&mut self) {
        let token = self.tokens.next().unwrap();
        self.consume(token);
    }

    #[allow(unused)]
    pub fn consume_while(&mut self, mut predicate: impl FnMut(&Token<'t>) -> bool) {
        while let Some(token) = self.tokens.peek() {
            if predicate(token) {
                self.consume_next();
            } else {
                return;
            }
        }
    }

    #[allow(unused)]
    pub fn consume_until_true_including(&mut self, mut predicate: impl FnMut(&Token<'t>) -> bool) {
        while let Some(token) = self.tokens.next() {
            let predicate = predicate(&token);
            self.consume(token);
            if !predicate {
                continue;
            } else {
                return;
            }
        }
    }

    pub fn expect(
        &mut self,
        predicate: impl ExpectPredicate,
        skip: &[TokenKind],
        stop_if: impl Fn(&Token) -> bool,
    ) -> bool {
        let mut error_fired = false;
        while let Some(token) = self.tokens.peek() {
            if !predicate.matches(&token) {
                if skip.contains(&token.kind) {
                    self.consume_next();
                    continue;
                }

                if stop_if(&token) {
                    if !error_fired {
                        // We failed to match a token we are looking for
                        // We need to emit at least one error
                        self.errors.push(predicate.error(&token, self.offset));
                    }
                    return false;
                }

                error_fired = true;
                self.errors.push(predicate.error(&token, self.offset));
                self.consume_next();
            } else {
                self.consume_next();
                return true;
            }
        }

        return false;
    }

    pub fn maybe(&mut self, kind: TokenKind) -> Option<Token<'t>> {
        if let Some(token) = self.tokens.peek() {
            if token.kind == kind {
                return Some(self.tokens.next().unwrap());
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn consume_trivia(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                trivia_with_newline!() => self.consume_next(),
                _ => return,
            }
        }
    }

    pub fn consume_trivia_until_nl(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                trivia!() => self.consume_next(),
                _ => return,
            }
        }
    }

    pub fn consume_maybe(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = self.maybe(kind) {
            self.consume(token);
            return true;
        } else {
            return false;
        }
    }

    pub fn unexpected(&mut self, token: Token<'t>) {
        self.builder.start_node(NodeKind::Error.into());
        self.consume(token);
        self.builder.finish_node();
    }

    pub fn expected(&mut self, expected: &[TokenKind], got: TokenKind) {
        self.errors.push(SyntaxError::new_at_offset(
            format!("Expected [{:?}], got {:?}", expected, got),
            self.offset,
        ))
    }
}

pub struct Parsed {
    pub schema: GreenNode,
    pub errors: Vec<SyntaxError>,
}

pub fn parse<'c, 't>(tokens: impl TokenIter<'t>, cache: &'c mut rowan::NodeCache) -> Parsed {
    let parser = Parser {
        builder: GreenNodeBuilder::with_cache(cache),
        tokens: tokens.peekable(),
        #[cfg(debug_assertions)]
        parsed: String::new(),
        offset: ByteOffset(0),
        _t: PhantomData,
        errors: Vec::new(),
    };

    parser.parse_schema()
}

pub fn parse_str<'a>(source: &'a str) -> Parsed {
    parse(tokenize(StrSource::new(source)), &mut NodeCache::default())
}
