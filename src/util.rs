pub trait CheckedMultiplication {
    fn checked_multiply(self, other: Self) -> Result<Self, &'static str> where Self: Sized;
}

impl CheckedMultiplication for u8 {
    fn checked_multiply(self, other: Self) -> Result<Self, &'static str> {
        match self.checked_mul(other) {
            Some(result) => Ok(result),
            None => Err("multiplication exceeds the u8 capacity"),
        }
    }
}