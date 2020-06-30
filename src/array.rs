use arraylib::{Array, ArrayExt};
use crate::{Nil, Cons};

pub trait IntoArray<T> {
    type Repr: Array;

    fn into_array(self) -> Self::Repr;
}

impl<T> IntoArray<T> for Nil {
    type Repr = [T; 0];

    fn into_array(self) -> Self::Repr {
        []
    }
}

impl<H, T, U> IntoArray<U> for Cons<H, T>
where
    H: Into<U>,
    T: IntoArray<U>,
    [U; <T as IntoArray<U>>::Repr::SIZE + 1]: Array,
{
    type Repr = [U; 1 + <T as IntoArray<U>>::Repr::SIZE];

    fn into_array(self) -> Self::Repr {
        [self.0].concat_arr(self.1.into_array())
    }
}

#[test]
fn into_array() {
    assert_eq!(
        hlist![1, 2, 3].into_array(),
        [1, 2, 3]
    );

    assert_eq!(
        Nil.into_array(),
        []
    );
}
