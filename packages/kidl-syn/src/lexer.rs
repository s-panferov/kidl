use std::{borrow::Cow, marker::PhantomData};

use crate::{helpers::ByteOffset, kind::TokenKind, source::Source};

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub slice: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, slice: impl Into<Cow<'a, str>>) -> Self {
        Token {
            kind,
            slice: slice.into(),
        }
    }
}

#[derive(Clone)]
pub struct Lexer<'a, S: Source<'a>> {
    source: S,
    offset: ByteOffset,
    consumed: ByteOffset,
    _a: PhantomData<&'a ()>,
}

impl<'a, S: Source<'a>> Lexer<'a, S> {
    fn consume_whitespace(&mut self) {
        while let Some(char) = self.source.peek() {
            if char.is_whitespace() {
                self.consume();
            } else {
                return;
            }
        }
    }

    fn consume_ident(&mut self) {
        while let Some(char) = self.source.peek() {
            if char.is_alphanumeric() || char == '_' {
                self.consume();
            } else {
                return;
            }
        }
    }

    fn consume(&mut self) -> Option<char> {
        self.source.next().map(|c| {
            self.consumed += ByteOffset(c.len_utf8());
            c
        })
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while let Some(char) = self.source.peek() {
            if predicate(char) {
                self.consume();
            } else {
                return;
            }
        }
    }

    fn consume_until_true_including(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while let Some(char) = self.consume() {
            if !predicate(char) {
                continue;
            } else {
                return;
            }
        }
    }
}

impl<'a, S: Source<'a>> Iterator for Lexer<'a, S> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.consume() {
            let kind = match c {
                '{' => TokenKind::CurlyOpen,
                '}' => TokenKind::CurlyClose,
                '[' => TokenKind::SquareOpen,
                ']' => TokenKind::SquareClose,
                '(' => TokenKind::ParenOpen,
                ')' => TokenKind::ParenClose,
                '<' => TokenKind::AngleOpen,
                '>' => TokenKind::AngleClose,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semicolon,
                ',' => TokenKind::Comma,
                '#' => TokenKind::Hash,
                '.' => TokenKind::Dot,
                '?' => TokenKind::Question,
                '=' => TokenKind::Eq,
                '\'' => {
                    self.consume_until_true_including(|c| c == '\'');
                    TokenKind::String
                }
                '\"' => {
                    self.consume_until_true_including(|c| c == '\"');
                    TokenKind::String
                }
                '/' if self.source.peek() == Some('/') => {
                    self.consume(); // consumes the second /
                    self.consume_until_true_including(|c| c == '\n');
                    TokenKind::Comment
                }
                '\n' => TokenKind::NewLine,
                c if c.is_whitespace() => {
                    self.consume_whitespace();
                    TokenKind::Space
                }
                c if c.is_numeric() => {
                    self.consume_while(|c| c.is_numeric());
                    TokenKind::Number
                }
                _ if c.is_alphabetic() => {
                    self.consume_ident();
                    TokenKind::Ident
                }
                c if unicode_ident::is_xid_start(c) => {
                    self.consume_while(|c| unicode_ident::is_xid_continue(c));
                    TokenKind::Ident
                }
                _ => todo!(),
            };

            let next_offset = self.offset + self.consumed;
            let range = self.offset.0..(next_offset.0);
            self.consumed.0 = 0;
            self.offset = next_offset;

            return Some(Token {
                kind,
                slice: self.source.slice(range),
            });
        } else {
            None
        }
    }
}

pub fn tokenize<'a, S: Source<'a>>(source: S) -> Lexer<'a, S> {
    Lexer {
        offset: ByteOffset(0),
        consumed: ByteOffset(0),
        _a: PhantomData,
        source,
    }
}
