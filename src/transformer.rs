use crate::monad::{Monad, Functor};

#[derive(Debug)]
#[non_exhaustive]
pub struct OptionT<M>(pub M);

impl<M: Functor> Functor for OptionT<M> {
    type Unwrapped = M::Unwrapped;
    type Wrapped<B> = OptionT<M::Wrapped<B>>;

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