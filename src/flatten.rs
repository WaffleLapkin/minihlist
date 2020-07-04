use crate::{Cons, Nil, Extend, HList};

///
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
pub trait Flatten {
    type Output;

    fn flatten(self) -> Self::Output;
}

impl<H, T> Flatten for Cons<H, T>
where
    Is<H>: NotHList,
    T: Flatten,
{
    type Output = Cons<H, T::Output>;

    fn flatten(self) -> Self::Output {
        let Cons(head, tail) = self;
        Cons(head, tail.flatten())
    }
}

impl<T> Flatten for Cons<Nil, T>
where
    T: Flatten,
{
    type Output = T::Output;

    fn flatten(self) -> Self::Output {
        self.1.flatten()
    }
}

impl<HH, HT, T> Flatten for Cons<Cons<HH, HT>, T>
where
    Cons<HH, HT>: Extend<T::Output>,
    T: Flatten,
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

/// Trait implemented for all types except `Is<`[`Cons`]`>` and `Is<`[`Nil`]`>`.
/// It is needed to implement traits differently for `HList`s and for other types.
/// (e.g.: [`Flatten`] is implemented using this trait)
///
/// [`Cons`]: crate::Cons
/// [`Nil`]: crate::Nil
auto trait NotHList {}
impl !NotHList for Is<Nil> {}
impl<H, T> !NotHList for Is<Cons<H, T>> {}

/// `Is` is being used to prevent false negative errors with types those include `Nil` or
/// `Cons<_, _>` (e.g.: `struct Test(Nil)`). See [#3] & test `hlist_with_hlist_inside_struct` down
/// below for more.
///
/// [#3]: https://github.com/WaffleLapkin/minihlist/issues/3
struct Is<T>(T);

#[test]
fn test() {
    use crate::hlist;

    let hlist = hlist![1, hlist![2, 3, 4], hlist![5, hlist![6, 7], hlist![8], 9]];
    assert_eq!(
        hlist.flatten().flatten(),
        hlist![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}

/// Test for issue [#3](https://github.com/WaffleLapkin/minihlist/issues/3)
#[test]
fn hlist_with_hlist_inside_struct() {
    use crate::hlist;

    #[derive(Debug, PartialEq)]
    struct Wrap(Nil);

    let hlist = hlist![Wrap(Nil), hlist![8]];
    assert_eq!(
        hlist.flatten(),
        hlist![Wrap(Nil), 8]
    );
}
