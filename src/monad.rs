use crate::monoid::{Monoid};

pub trait Functor {
    type A;
    type FB<B>;
    fn fmap<B, F>(self, f:F) -> Self::FB<B>
        where F: Fn(Self::A) -> B; 
}


pub trait Monad: Functor {
    type Unwrapped;
    type Wrapped<B>: Monad;

    fn unit(x: Self::Unwrapped) -> Self;

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B>;
}

#[derive(Debug)]
pub struct Writer<A, W: Monoid>(pub A, pub W);

impl<A, W: Monoid> Functor for Writer<A, W> {
    type A = A;
    type FB<B> = Writer<B, W>;
    fn fmap<B, F>(self, f:F) -> Self::FB<B>
        where F: Fn(Self::A) -> B {
        Writer(f(self.0), self.1)
    }
}

impl<A, W: Monoid> Monad for Writer<A, W> {
    type Unwrapped = A;
    type Wrapped<B> = Writer<B, W>;

    fn unit(x: Self::Unwrapped) -> Self {
        Self(x, W::new())
    }

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B>, Self: Sized {
        let Writer(a, mut w) = f(self.0);
        Writer(a, w.append(self.1))
    }
}

impl<A> Functor for Option<A> {
    type A = A;
    type FB<B> = Option<B>;

    fn fmap<B, F>(self, f:F) -> Self::FB<B>
        where F: Fn(Self::A) -> B {
        self.map(f)
    }
}

impl<A> Monad for Option<A> {
    type Unwrapped = A;
    type Wrapped<B> = Option<B>;

    fn unit(x: Self::Unwrapped) -> Self {
        Some(x)
    }

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B> where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        self.and_then(f)
    }
}

impl<A, E> Functor for Result<A, E> {
    type A = A;
    type FB<B> = Result<B, E>;

    fn fmap<B, F>(self, f:F) -> Self::FB<B>
        where F: Fn(Self::A) -> B {
        self.map(f)
    }
}

impl<A, E> Monad for Result<A, E> {
    type Unwrapped = A;
    type Wrapped<B> = Result<B, E>;
    fn unit(x: Self::Unwrapped) -> Self {
        Ok(x)
    }

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B> where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        self.and_then(f)
    }
}