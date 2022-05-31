use crate::monoid::{Monoid};

pub trait Monad {
    type Unwrapped;
    type Wrapped<B>: Monad;

    fn unit(x: Self::Unwrapped) -> Self;

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B>;
}

#[derive(Debug)]
pub struct Writer<A, W: Monoid>(pub A, pub W);

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