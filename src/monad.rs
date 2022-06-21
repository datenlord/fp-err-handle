use crate::monoid::Monoid;

pub trait Functor {
    type Unwrapped;
    type Wrapped<B>: Functor;
<<<<<<< HEAD
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B; 
=======
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B;
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
}

pub trait Monad: Functor {
<<<<<<< HEAD

=======
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    fn unit(x: Self::Unwrapped) -> Self;

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>;
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Writer<A, W: Monoid>(pub A, pub W);

impl<A, W: Monoid> Functor for Writer<A, W> {
    type Unwrapped = A;
    type Wrapped<B> = Writer<B, W>;

    #[inline]
<<<<<<< HEAD
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
=======
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
        Writer(f(self.0), self.1)
    }    
}

impl<A, W: Monoid> Monad for Writer<A, W> {
<<<<<<< HEAD

=======
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Self(x, W::mempty())
    }

    #[inline]
<<<<<<< HEAD
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        let Writer(a, mut w) = f(self.0);
        Writer(a, w.mappend(self.1))
    }    
=======
    fn bind<B, F>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        let Writer(a, mut w) = f(self.0);
        Writer(a, w.mappend(self.1))
    }
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
}

impl<A> Functor for Option<A> {
    type Unwrapped = A;
    type Wrapped<B> = Option<B>;
<<<<<<< HEAD
    
    #[inline]
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
=======

    #[inline]
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
        self.map(f)
    }
}

impl<A> Monad for Option<A> {
<<<<<<< HEAD

=======
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Some(x)
    }

    #[inline]
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
<<<<<<< HEAD
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
=======
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
        self.and_then(f)
    }
}

impl<A, E> Functor for Result<A, E> {
    type Unwrapped = A;
    type Wrapped<B> = Result<B, E>;

    #[inline]
<<<<<<< HEAD
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
=======
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
        self.map(f)
    }
}

impl<A, E> Monad for Result<A, E> {
<<<<<<< HEAD

=======
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Ok(x)
    }

    #[inline]
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
<<<<<<< HEAD
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
=======
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
        self.and_then(f)
    }
}

impl<A> Functor for Vec<A> {
    type Unwrapped = A;
    type Wrapped<B> = Vec<B>;

    #[inline]
<<<<<<< HEAD
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
=======
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
        self.into_iter().map(f).collect()
    }
}

impl<A> Monad for Vec<A> {
<<<<<<< HEAD

=======
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        vec![x]
    }

    #[inline]
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
<<<<<<< HEAD
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        self.into_iter().flat_map(f).collect()
    }
}
=======
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        self.into_iter().flat_map(f).collect()
    }
}
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
