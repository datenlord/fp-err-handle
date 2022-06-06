use crate::monad::Monad;


pub trait MonadTrans {
    type Item;
    type Monad: Monad;
    type Transformer<B>: MonadTrans;

    fn unit(x: Self::Item) -> Self;

    fn bind<B, F>(self, f: F) -> Self::Transformer<B>
        where F: Fn(Self::Item) -> Self::Transformer<B>;

}
#[derive(Debug)]
pub struct OptionT<M: Monad>(M);

impl<A, M> MonadTrans for OptionT<M> 
where M: Monad<Unwrapped = A>
{
    type Item = A;
    type Monad = M;
    type Transformer<B> = OptionT<<M as Monad>::Wrapped<B>>;

    #[inline]
    fn unit(x: Self::Item) -> Self {
        Self(M::unit(x))
    }

    #[inline]
    fn bind<B, F>(self, f: F) -> Self::Transformer<B>
        where F: Fn(Self::Item) -> Self::Transformer<B> {
            OptionT(self.0.bind(|a| f(a).0))
    }
}


#[cfg(test)]
mod tests {
    use crate::monad::Monad;

    use super::{OptionT, MonadTrans};

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