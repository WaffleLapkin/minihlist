use crate::{Cons, Nil};

pub trait Extend<T> {
    type Output;

    fn extend(self, val: T) -> Self::Output;
}

impl<A, H, T> Extend<T> for Cons<A, H>
where
    H: Extend<T>,
{
    type Output = Cons<A, H::Output>;

    fn extend(self, val: T) -> Self::Output {
        Cons(self.0, self.1.extend(val))
    }
}

impl<T> Extend<T> for Nil {
    type Output = T;

    fn extend(self, val: T) -> Self::Output {
        val
    }
}
