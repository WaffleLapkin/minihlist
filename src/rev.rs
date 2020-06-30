use crate::{Nil, Cons, hlist, Extend};

/// Reverse hlist.
///
/// ## Examples
///
/// ```
/// use minihlist::{hlist, Rev};
///
/// let list = hlist![1, "h", 'x'];
/// assert_eq!(list.rev(), hlist!['x', "h", 1])
/// ```
pub trait Rev {
    type Output;

    fn rev(self) -> Self::Output;
}

impl Rev for Nil {
    type Output = Nil;

    fn rev(self) -> Self::Output { self }
}

impl<H, T> Rev for Cons<H, T>
where
    T: Rev,
    T::Output: Extend<Cons<H, Nil>>,
{
    type Output = <T::Output as Extend<Cons<H, Nil>>>::Output;

    fn rev(self) -> Self::Output {
        self.1.rev().extend(hlist![self.0])
    }
}
