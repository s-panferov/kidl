use std::{iter::Peekable, marker::PhantomData};

use rowan::{GreenNode, GreenNodeBuilder};

use crate::{
    kind::{NodeKind, TokenKind},
    lexer::{tokenize, Token},
    source::StrSource,
};

pub use rowan::NodeCache;

pub trait TokenIter<'t>: Iterator<Item = Token<'t>> {}
impl<'t, T> TokenIter<'t> for T where T: Iterator<Item = Token<'t>> {}

pub struct Parser<'c, 't, T: TokenIter<'t>> {
    pub(crate) builder: GreenNodeBuilder<'c>,
    pub(crate) tokens: Peekable<T>,
    #[cfg(debug_assertions)]
    parsed: String,
    _t: PhantomData<&'t ()>,
}

impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub fn consume(&mut self, token: Token<'t>) {
        #[cfg(debug_assertions)]
        self.parsed.push_str(&token.slice);
        self.builder.token(token.kind.into(), &token.slice)
    }

    pub fn consume_next(&mut self) {
        let token = self.tokens.next().unwrap();
        #[cfg(debug_assertions)]
        self.parsed.push_str(&token.slice);
        self.builder.token(token.kind.into(), &token.slice)
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

    pub fn expect(&mut self, mut predicate: impl FnMut(&Token<'t>) -> bool) {
        let mut error = false;
        while let Some(token) = self.tokens.next() {
            let predicate = predicate(&token);
            if !predicate {
                if let TokenKind::Space | TokenKind::Comment | TokenKind::NewLine = token.kind {
                    self.consume(token);
                    continue;
                }

                if !error {
                    error = true;
                    self.builder.start_node(NodeKind::Error.into());
                    self.consume(token);
                }
            } else {
                if error {
                    self.builder.finish_node();
                }
                self.consume(token);
                return;
            }
        }

        // Make sure we close opened error node on EOL
        if error {
            self.builder.finish_node()
        }
    }

    #[inline]
    pub fn expect_token(&mut self, kind: TokenKind) {
        self.expect(|t| t.kind == kind)
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
}

pub fn parse<'c, 't>(tokens: impl TokenIter<'t>, cache: &'c mut rowan::NodeCache) -> GreenNode {
    let parser = Parser {
        builder: GreenNodeBuilder::with_cache(cache),
        tokens: tokens.peekable(),
        #[cfg(debug_assertions)]
        parsed: String::new(),
        _t: PhantomData,
    };

    parser.parse_schema()
}

pub fn parse_str<'a>(source: &'a str) -> GreenNode {
    parse(tokenize(StrSource::new(source)), &mut NodeCache::default())
}
