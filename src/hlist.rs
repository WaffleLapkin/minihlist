use crate::{Cons, Nil, Rev};

/// A marker trait that `Nil` and `Cons<H, T>` satisfies.
/// Not currently used to enforce proper hlists, although this may change.
/// Provides the `push()` method
pub trait HList: Sized + Rev {
    /// Consumes the `HList`, and returns a new HList with `item` at the beginning.
    fn push<N>(self, item: N) -> Cons<N, Self> {
        Cons(item, self)
    }
}

impl HList for Nil {}
impl<H, T> HList for Cons<H, T>
where
    Self: Rev,
{}
