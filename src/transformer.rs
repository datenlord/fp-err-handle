use crate::monad::Monad;


pub trait MonadTrans {
    type Item;
    type Monad: Monad;
    type Transformer<B>: MonadTrans;

    fn unit(x: Self::Item) -> Self;

    fn bind<B, F>(self, f: F) -> Self::Transformer<B>
        where F: Fn(Self::Item) -> Self::Transformer<B>;

}

pub struct OptionT<M: Monad>(M);

impl<A, M> MonadTrans for OptionT<M> 
where M: Monad<Unwrapped = A>
{
    type Item = A;
    type Monad = M;
    type Transformer<B> = OptionT<<M as Monad>::Wrapped<B>>;

    fn unit(x: Self::Item) -> Self {
        OptionT(M::unit(x))
    }

    fn bind<B, F>(self, f: F) -> Self::Transformer<B>
        where F: Fn(Self::Item) -> Self::Transformer<B> {
            OptionT(self.0.bind(|a| f(a).0))
    }
}


