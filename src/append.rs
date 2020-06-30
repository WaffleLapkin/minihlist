use crate::{Nil, Cons, hlist};

/// Append operation on hlist
pub trait Append<T> {
    /// Output type of the append operation
    type Output;

    /// Append a value at the end of the hlist
    ///
    /// ## Examples
    ///
    /// ```
    /// use minihlist::{hlist, Append};
    ///
    /// assert_eq!(hlist![1, ""].append(17), hlist![1, "", 17]);
    /// assert_eq!(hlist![].append(42), hlist![42]);
    /// ```
    fn append(self, val: T) -> Self::Output;
}

impl<T> Append<T> for Nil {
    type Output = Cons<T, Nil>;

    fn append(self, val: T) -> Self::Output {
        hlist![val]
    }
}

impl<E, H, T> Append<E> for Cons<H, T>
where
    T: Append<E>,
{
    type Output = Cons<H, T::Output>;

    fn append(self, val: E) -> Self::Output {
        Cons(self.0, self.1.append(val))
    }
}
