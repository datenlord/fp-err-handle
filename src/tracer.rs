use crate::{
    monad::{Functor, Monad},
    monoid::Monoid,
};
use backtrace::Backtrace;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Log<E: Display + Clone> {
    backtrace: Backtrace,
    error: E,
}

impl<E: Display + Clone> Display for Log<E> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}, BackTrace\n {:?}", self.error, self.backtrace)
    }
}

impl<E: Display + Clone> Log<E> {
    #[inline]
    pub fn new(backtrace: Backtrace, error: E) -> Self {
        Self { backtrace, error }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Tracer<M, E: Display + Clone>(pub M, pub Vec<Log<E>>);

impl<M: Functor, E: Display + Clone> Functor for Tracer<M, E> {
    type Unwrapped = M::Unwrapped;
    type Wrapped<B> = Tracer<M::Wrapped<B>, E>;

    #[inline]
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        Tracer(self.0.fmap(f), self.1)
    }
}

impl<M: Monad, E: Display + Clone> Monad for Tracer<M, E> {
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Self(M::unit(x), Vec::mempty())
    }

    #[inline]
    fn bind<B, F>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        let mut v = self.1;
        let m = self.0.bind(|x| {
            let t = f(x);
            v = t.1;
            t.0
        });

        Tracer(m, v)
    }
}

impl<A, E: Display + Clone> Tracer<Result<A, E>, E> {
    #[inline]
    pub fn lift(r: Result<A, E>) -> Self {
        match r {
            Ok(x) => Tracer(Ok(x), Vec::mempty()),
            Err(e) => Tracer(
                Err(e.clone()),
                Vec::mempty().mappend(vec![Log::new(Backtrace::new(), e)]),
            ),
        }
    }

    #[inline]
    pub fn map<B, F>(mut self, f: F) -> Tracer<Result<B, E>, E>
    where
        F: Fn(Result<A, E>) -> Result<B, E>,
    {
        let is_err = self.0.is_err();
        let r = f(self.0);

        if is_err {
            return Tracer(r, self.1);
        }

        match r {
            Ok(x) => Tracer(Ok(x), self.1),
            Err(e) => Tracer(
                Err(e.clone()),
                self.1.mappend(vec![Log::new(Backtrace::new(), e)]),
            ),
        }
    }

    #[inline]
    pub fn log(&mut self, e: E) {
        self.1.push(Log::new(Backtrace::new(), e));
    }
}

pub trait Sequence {
    type Value;
    type Err;

    fn sequence(self) -> Result<Vec<Self::Value>, Self::Err>;
}

impl<A, E: Display + Clone> Sequence for Vec<Tracer<Result<A, E>, E>> {
    type Value = A;
    type Err = E;
    fn sequence(self) -> Result<Vec<Self::Value>, Self::Err> {
        let list = vec![];
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::Tracer;
    use crate::monad::Monad;

    #[test]
    fn it_works() {
        let z: Tracer<Result<i32, _>, _> = Tracer::lift(Err(" ".to_string()));
        println!("{:?}", z);

        let y = Tracer::unit("a").map(|x| x.unwrap().parse::<i32>());
        println!("{:?}", y);

        let mut x = Tracer::unit(2);
        x.log("I'm an error");
        println!("{:?}", x);
    }
}
