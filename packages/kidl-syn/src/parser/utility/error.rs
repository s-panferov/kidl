use std::{fmt, ops::Range};

use crate::helpers::ByteOffset;

/// Represents the result of unsuccessful tokenization, parsing
/// or tree validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(String, Range<ByteOffset>);

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: Range<ByteOffset>) -> Self {
        Self(message.into(), range)
    }

    pub fn new_at_offset(message: impl Into<String>, offset: ByteOffset) -> Self {
        Self(
            message.into(),
            Range {
                start: offset,
                end: offset,
            },
        )
    }

    pub fn range(&self) -> Range<ByteOffset> {
        self.1.clone()
    }

    pub fn with_range(mut self, range: Range<ByteOffset>) -> Self {
        self.1 = range;
        self
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
