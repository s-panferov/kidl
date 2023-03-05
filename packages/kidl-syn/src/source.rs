use std::{borrow::Cow, iter::Peekable, ops::Range, str::Chars};

use ropey::RopeSlice;

pub trait Source<'a>: Clone {
    fn next(&mut self) -> Option<char>;
    fn peek(&mut self) -> Option<char>;
    fn slice(&self, range: Range<usize>) -> Cow<'a, str>;
}

#[derive(Clone)]

pub struct StrSource<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> StrSource<'a> {
    #[allow(unused)]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
        }
    }
}

impl<'a> Source<'a> for StrSource<'a> {
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn slice(&self, range: Range<usize>) -> Cow<'a, str> {
        Cow::Borrowed(&self.source[range])
    }
}

#[derive(Clone)]

pub struct RopeSource<'a> {
    source: RopeSlice<'a>,
    chars: Peekable<ropey::iter::Chars<'a>>,
}

impl<'a> RopeSource<'a> {
    #[allow(unused)]
    pub fn new(source: RopeSlice<'a>) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
        }
    }
}

impl<'a> Source<'a> for RopeSource<'a> {
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn slice(&self, range: Range<usize>) -> Cow<'a, str> {
        self.source.byte_slice(range).into()
    }
}
