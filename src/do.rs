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

    #[test]
    fn it_works() {
        let x = _mdo! {
          x <- Ok("a");
          y <- x.parse::<i32>();
          Ok(y)
        };

        println!("{:?}", x);

        let y = _mdo! {
            i <- OptionT(Ok("5"));
            j <- OptionT(Ok(i.parse::<i32>()));
            OptionT(j)
        };

        println!("{:?}", y);
    }
}
