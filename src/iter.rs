use std::fmt::Display;

use crate::{monad::{Monad, Writer}};

pub struct MapLog<I, F> {
    iter: I,
    f: F,
}

impl<I, F> MapLog<I, F> {

    #[inline]
    pub const fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

pub trait TryMap: Iterator {

    #[inline]
    fn try_map<B, F>(self, f: F) -> MapLog<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        MapLog::new(self, f)
    }
}

impl<I: Iterator> TryMap for I {}

pub trait TryCollect {
    type M: Monad;
    fn try_collect(self) -> Self::M;
}

impl<B, E, I, F> TryCollect for MapLog<I, F> 
where E: Display, I: Iterator, F: FnMut(I::Item) -> Result<B, E>
{
    type M = Writer<Vec<B>, String>;

    #[inline]
    fn try_collect(mut self) -> Self::M {
        let mut w: Writer<Vec<B>, String> = Writer::unit(Vec::new());
        for (i, x) in self.iter.enumerate() {
            match (self.f)(x) {
                Ok(a) => { w.0.push(a); },
                Err(e) => { 
                    let s = format!("\nError {} occurs on index {}", e, i);
                    w.1.push_str(&s); //append(s);
                },
            };
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::{TryMap, TryCollect};

    #[test]
    fn it_works() {
       let r = ["1", "a", "why", "2"].iter().try_map(|x| x.parse::<i32>()).try_collect();
       println!("{:?}", r.0);
       println!("{}", r.1);
    }
}