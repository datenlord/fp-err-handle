use std::fmt::Display;

use crate::{monad::{Functor, Monad}, monoid::Monoid};

#[macro_export] 
macro_rules! log {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        Log {
            func: name[..name.len() - 3].to_string(),
            file: file!().to_string(),
            line: line!(),
        }
    }}
}

#[derive(Debug, Clone)]
pub struct Log {
    func: String,
    file: String,
    line: u32,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File {}, Line {}, Func: {}", self.file, self.line, self.func)
    }
}

impl Log {
    pub fn new() -> Self {
        Self { 
            func: String::mempty(),
            file: String::mempty(), 
            line: 0 
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tracer<A, E>(pub Result<A, E>, pub Vec<Log>);

impl<A, E> Functor for Tracer<A, E> {
    type Unwrapped = A;
    type Wrapped<B> = Tracer<B, E>;

    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
        Tracer(self.0.fmap(f), self.1)
    }
}

impl<A, E> Monad for Tracer<A, E> {
    fn unit(x: Self::Unwrapped) -> Self {
        Self(Ok(x), Vec::mempty())
    }

    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        match self.0 {
            Ok(x) => f(x),
            Err(e) => Tracer(Err(e), self.1),
        }
    }
}

impl<A, E> Tracer<A, E> {
    pub fn lift(r: Result<A, E>) -> Self {
        Tracer(r, Vec::mempty())
    }

    pub fn trace(mut self, log: Log) -> Self {
        self.1.push(log);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{monad::Monad, tracer::Log};
    use super::Tracer;

    #[test]
    fn it_works() {
        println!("{}", log!());
        let z: Tracer<i32, String> = Tracer::lift(Err(" ".to_string()));
        println!("{:?}", z);
        
        let y: Tracer<i32, String> = Tracer::unit(5).trace(log!());
        println!("{:?}", y);

        let x: Tracer<i32, String> = Tracer::unit(2);
        println!("{:?}", x);
    }
}