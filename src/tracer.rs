use crate::monad::Monad;
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
#[deny(warnings)]
pub trait Tracer: Monad {
    type WrappedLog;

    fn log(self) -> Self::WrappedLog;
}

impl<A, E: Display + Clone> Tracer for Result<A, E> {
    type WrappedLog = Result<A, Log<E>>;

    #[inline]
    fn log(self) -> Self::WrappedLog {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Log::new(Backtrace::new(), e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tracer;

    #[test]
    fn it_works() {
        let x: Result<i32, i32> = Err(2);
        println!("{:?}", x.log());
    }
}
