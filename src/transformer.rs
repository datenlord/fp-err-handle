<<<<<<< HEAD
use crate::monad::{Monad, Functor};
=======
use crate::monad::{Functor, Monad};
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e

#[derive(Debug)]
#[non_exhaustive]
pub struct OptionT<M>(pub M);

impl<M: Functor> Functor for OptionT<M> {
    type Unwrapped = M::Unwrapped;
    type Wrapped<B> = OptionT<M::Wrapped<B>>;
<<<<<<< HEAD

    #[inline]
    fn fmap<B, F>(self, f:F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> B {
        OptionT(self.0.fmap(f))
    }
}


impl<M: Monad> Monad for OptionT<M> {

    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Self(M::unit(x))
    }

    #[inline]
    fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
        where F: Fn(Self::Unwrapped) -> Self::Wrapped<B> {
        OptionT(self.0.bind(|x| f(x).0))
    }
}


#[cfg(test)]
mod tests {
    use crate::{transformer::OptionT, monad::Monad};

    #[test]
    fn it_works() {
       let r = OptionT(Some(5)).bind(|a| OptionT(Some(a+1)));
       println!("{:?}", r);

       let vec: Vec<_> = (1..=5).map(|a| Some(a)).collect();
       println!("{:?}", vec);

       let r2 = OptionT(vec).bind(|a| OptionT(vec![a.bind(|x| Some(x+1))]));
       println!("{:?}", r2);
    }
}
=======

    #[inline]
    fn fmap<B, F>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        OptionT(self.0.fmap(f))
    }
}

impl<M: Monad> Monad for OptionT<M> {
    #[inline]
    fn unit(x: Self::Unwrapped) -> Self {
        Self(M::unit(x))
    }

    #[inline]
    fn bind<B, F>(self, mut f: F) -> Self::Wrapped<B>
    where
        F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
    {
        OptionT(self.0.bind(|x| f(x).0))
    }
}

#[cfg(test)]
mod tests {
    use crate::{monad::Monad, transformer::OptionT};

    #[test]
    fn it_works() {
        let r = OptionT(Some(5)).bind(|a| OptionT(Some(a + 1)));
        println!("{:?}", r);

        let vec: Vec<_> = (1..=5).map(|a| Some(a)).collect();
        println!("{:?}", vec);

        let r2 = OptionT(vec).bind(|a| OptionT(vec![a.bind(|x| Some(x + 1))]));
        println!("{:?}", r2);
    }
}
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
