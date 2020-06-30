use crate::{Cons, Nil, Extend};

/// Trait implemented for all types except [`Cons`] and [`Nil`].
/// It is needed to implement traits differently for `HList`s and for other types.
/// (e.g.: [`Flatten`] is implemented using this trait)
///
/// [`Cons`]: crate::Cons
/// [`Nil`]: crate::Nil
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
pub auto trait NotHList {}
impl !NotHList for Nil {}
impl<H, T> !NotHList for Cons<H, T> {}

///
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
pub trait Flatten {
    type Output;

    fn flatten(self) -> Self::Output;
}

impl<H: NotHList, T: Flatten> Flatten for Cons<H, T> {
    type Output = Cons<H, T::Output>;

    fn flatten(self) -> Self::Output {
        let Cons(head, tail) = self;
        Cons(head, tail.flatten())
    }
}

impl<T: Flatten> Flatten for Cons<Nil, T> {
    type Output = T::Output;

    fn flatten(self) -> Self::Output {
        self.1.flatten()
    }
}

impl<HH, HT, T: Flatten> Flatten for Cons<Cons<HH, HT>, T>
where
    Cons<HH, HT>: Extend<T::Output>
{
    type Output = <Cons<HH, HT> as Extend<T::Output>>::Output;

    fn flatten(self) -> Self::Output {
        let Cons(head, tail) = self;
        head.extend(tail.flatten())
    }
}

impl Flatten for Nil {
    type Output = Nil;

    fn flatten(self) -> Self::Output {
        self
    }
}

#[test]
fn test() {
    use crate::hlist;

    let hlist = hlist![1, hlist![2, 3, 4], hlist![5, hlist![6, 7], hlist![8], 9]];
    assert_eq!(
        hlist.flatten().flatten(),
        hlist![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}