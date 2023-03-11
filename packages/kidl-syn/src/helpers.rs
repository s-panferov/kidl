use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteOffset(pub usize);

impl Add<ByteOffset> for ByteOffset {
    type Output = ByteOffset;

    fn add(self, rhs: ByteOffset) -> Self::Output {
        ByteOffset(self.0 + rhs.0)
    }
}

impl AddAssign<ByteOffset> for ByteOffset {
    fn add_assign(&mut self, rhs: ByteOffset) {
        self.0 += rhs.0;
    }
}

impl Debug for ByteOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Byte({})", self.0)
    }
}
