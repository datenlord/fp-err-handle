use std::fmt::{Display, Debug};
use backtrace::Backtrace;
use crate::{monad::{Functor, Monad}, monoid::Monoid};

#[derive(Debug, Clone)]
pub struct Log<E: Display + Clone> {
    backtrace: Backtrace,
    error: E
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
        Self { 
            backtrace,
            error
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Tracer<A, E: Display + Clone>(pub Result<A, E>, pub Vec<Log<E>>);

impl<A, E: Display + Clone> Functor for Tracer<A, E> {
    type Unwrapped = A;
    type Wrapped<B> = Tracer<B, E>;

    #[inline]
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
        Tracer(self.0.fmap(f), self.1)
    }
}

impl<A, E: Display + Clone> Monad for Tracer<A, E> {
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Self(Ok(x), Vec::mempty())
    }

    #[inline]
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        match self.0 {
            Ok(x) => f(x),
            Err(e) => Tracer(Err(e), self.1),
        }
    }
}

impl<A, E: Display + Clone> Tracer<A, E> {
    #[inline]
    pub fn lift(r: Result<A, E>) -> Self {
        match r {
            Ok(x) => Tracer(Ok(x), Vec::mempty()),
            Err(e) => Tracer(
                Err(e.clone()), 
                Vec::mempty().mappend(vec![Log::new(Backtrace::new(), e)])
            ),
        }
    }

    #[inline]
    pub fn map<B, F>(mut self, f:F) -> Tracer<B, E>
        where F: Fn(Result<A, E>) -> Result<B, E> {
        let is_err = self.0.is_err();
        let r = f(self.0); 

        if is_err { return Tracer(r, self.1); }

        match r {
            Ok(x) => Tracer(Ok(x), self.1),
            Err(e) => Tracer(
                Err(e.clone()), 
                self.1.mappend(vec![Log::new(Backtrace::new(), e)])
            ),
        }
    }

    #[inline]
    pub fn log(&mut self, e: E) {
        self.1.push(Log::new(Backtrace::new(), e));
    }
}

#[cfg(test)]
mod tests {
    use crate::monad::Monad;
    use super::Tracer;

    #[test]
    fn it_works() {
        let z: Tracer<i32, String> = Tracer::lift(Err(" ".to_string()));
        println!("{:?}", z);
        
        let y = Tracer::unit("a").map(|x| x.unwrap().parse::<i32>());
        println!("{:?}", y);

        let mut x = Tracer::unit(2);
        x.log("I'm an error");
        println!("{:?}", x);
    }
}