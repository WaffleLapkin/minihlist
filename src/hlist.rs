use crate::{Cons, Nil, Rev};

/// A marker trait that `Nil` and `Cons<H, T>` satisfies.
/// Not currently used to enforce proper hlists, although this may change.
/// Provides the `push()` method
pub trait HList: Sized + Rev /* <-- Anyone knows why I've added this bound here? Because I don't */ {
    /// The lenght of the list
    ///
    /// ```
    /// use minihlist::HList;
    ///
    /// assert_eq!(<HList![bool, i32]>::LEN, 2);
    /// assert_eq!(<HList![]>::LEN, 0);
    /// ```
    const LEN: usize;

    /// Convenient alias for `Self::LEN`
    ///
    /// ```
    /// use minihlist::{HList, hlist};
    ///
    /// assert_eq!(hlist![true, 0].len(), 2);
    /// assert_eq!(hlist![].len(), 0);
    /// ```
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Consumes the `HList`, and returns a new HList with `item` at the beginning.
    fn push<N>(self, item: N) -> Cons<N, Self> {
        Cons(item, self)
    }
}

impl HList for Nil {
    const LEN: usize = 0;
}

impl<H, T> HList for Cons<H, T>
where
    T: HList,
    Self: Rev,
{
    const LEN: usize = T::LEN + 1;
}
