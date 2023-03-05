use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
pub struct Bytes(pub usize);

impl Add<Bytes> for Bytes {
    type Output = Bytes;

    fn add(self, rhs: Bytes) -> Self::Output {
        Bytes(self.0 + rhs.0)
    }
}

impl AddAssign<Bytes> for Bytes {
    fn add_assign(&mut self, rhs: Bytes) {
        self.0 += rhs.0;
    }
}
