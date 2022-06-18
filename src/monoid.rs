pub trait Monoid {
    fn mempty() -> Self;

    #[must_use]
    fn mappend(&mut self, other: Self) -> Self;
}

impl Monoid for String {

    #[inline]
    fn mempty() -> Self {
        Self::new()
    }
    
    #[inline]
    #[must_use]
    fn mappend(&mut self, other: Self) -> Self {
        self.push_str(&other);
        self.clone()
    }
}

impl<T: Clone> Monoid for Vec<T> {
    fn mempty() -> Self {
        Self::new()
    }

    fn mappend(&mut self, other: Self) -> Self {
        self.append(&mut other.clone());
        self.to_vec()
    }
}