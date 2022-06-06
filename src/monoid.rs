pub trait Monoid {
    fn new() -> Self;

    #[must_use]
    fn append(&mut self, other: Self) -> Self;
}

impl Monoid for String {

    #[inline]
    fn new() -> Self {
        Self::new()
    }
    
    #[inline]
    #[must_use]
    fn append(&mut self, other: Self) -> Self {
        self.push_str(&other);
        self.clone()
    }
}