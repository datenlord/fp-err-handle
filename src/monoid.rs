pub trait Monoid {
    fn mempty() -> Self;

    #[must_use]
    fn mappend(&mut self, other: Self) -> Self;
}

impl Monoid for String {
<<<<<<< HEAD

=======
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    #[inline]
    fn mempty() -> Self {
        Self::new()
    }
<<<<<<< HEAD
    
=======

>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    #[inline]
    #[must_use]
    fn mappend(&mut self, other: Self) -> Self {
        self.push_str(&other);
        self.clone()
<<<<<<< HEAD
=======
    }
}

impl<T: Clone> Monoid for Vec<T> {
    #[inline]
    fn mempty() -> Self {
        Self::new()
    }

    #[inline]
    fn mappend(&mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self.clone()
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    }
}
