use crate::{Cons, Nil};
use std::ops::Add;
use typenum::{Unsigned, U0, U1};

#[cfg_attr(docsrs, doc(cfg(feature = "typenum")))]
#[allow(clippy::len_without_is_empty)]
pub trait Len {
    type Len: Unsigned;

    const LEN: usize = Self::Len::USIZE;

    #[inline]
    fn len(&self) -> usize {
        Self::Len::USIZE
    }
}

impl<H, T> Len for Cons<H, T>
where
    T: Len,
    T::Len: Add<U1>,
    <T::Len as Add<U1>>::Output: Unsigned,
{
    type Len = <T::Len as Add<U1>>::Output;
}

impl Len for Nil {
    type Len = U0;
}
