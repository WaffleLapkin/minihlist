use crate::{
    succnum::{Succ, Zero},
    Cons, Get,
};

/// Type-directed search & remove in hlist.
///
/// This trait is very simlar to [`Get`](crate::Get) but instead of borrowing, it allows to remove
/// elements from a hlist just by type.
///
/// ## Examples
///
/// ```
/// use minihlist::{hlist, Remove};
///
/// let mut list = hlist![1, 'x'];
///
/// let (i, rest): (i32, _) = list.remove();
/// assert_eq!(i, 1);
/// assert_eq!(rest, hlist!['x']);
/// ```
///
/// Note that just like with [`Get`] you can't remove elements those are not unique:
/// ```compile_fail,E0282
/// use minihlist::{hlist, Remove};
///
/// let list = hlist![17, 42];
/// let (_, _): (i32, _) = list.remove();
/// ```
pub trait Remove<Idx, Out>: Get<Idx, Out> {
    type Rest;

    fn remove(self) -> (Out, Self::Rest);
}

impl<H, T> Remove<Zero, H> for Cons<H, T> {
    type Rest = T;

    fn remove(self) -> (H, Self::Rest) {
        let Cons(head, tail) = self;
        (head, tail)
    }
}

impl<H, T, Idx, Out> Remove<Succ<Idx>, Out> for Cons<H, T>
where
    T: Remove<Idx, Out>,
{
    type Rest = Cons<H, T::Rest>;

    fn remove(self) -> (Out, Self::Rest) {
        let Cons(head, tail) = self;
        let (ret, tail_rest) = tail.remove();
        (ret, Cons(head, tail_rest))
    }
}
