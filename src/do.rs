macro_rules! _mdo {
    (_ <- $x:expr ; $($r:tt)*) => {
        $x.bind(move |_| { _mdo!($($r)*) })
    };

    ($a:ident <- $x:expr ; $($r:tt)*) => {
        $x.bind(move |$a| { _mdo!($($r)*) })
    };

    ($a:expr) => {
        $a
    }
}

#[cfg(test)]
mod tests {
    use crate::{monad::Monad, transformer::OptionT};
<<<<<<< HEAD
    
    #[test]
    fn it_works() {
        let x = _mdo!{
=======

    #[test]
    fn it_works() {
        let x = _mdo! {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
          x <- Ok("a");
          y <- x.parse::<i32>();
          Ok(y)
        };

        println!("{:?}", x);

<<<<<<< HEAD
        let y = _mdo!{
=======
        let y = _mdo! {
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
            x <- OptionT(Some(5));
            y <- OptionT(Some(x+1));
            OptionT(Some(y*2))
        };

        println!("{:?}", y);
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> 32dd15430f98ddfa21e1d527bf8be6da80c1337e
