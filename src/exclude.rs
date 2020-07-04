use crate::{Cons, Nil};

/// Marker trait that indicates that `HList` doesn't include type `T`
///
/// ## Examples
///
/// ```
/// use minihlist::{HList, Exclude, Nil};
/// fn assert_ty<L: Exclude<E>, E>() {}
///
/// assert_ty::<HList![usize, String, char], i32>();
/// assert_ty::<HList![u32, i32], Nil>();
/// ```
///
/// ```compile_fail,E0277
/// use minihlist::{HList, Exclude};
/// fn assert_ty<L: Exclude<E>, E>() {}
///
/// // fails to compile
/// assert_ty::<HList![u32, i32], i32>();
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
pub trait Exclude<T> {}

impl<T> Exclude<T> for Nil {}

impl<H, T, E> Exclude<E> for Cons<H, T>
where
    (H, E): private::TypeNeq,
    T: Exclude<E>,
{
}

mod private {
    /// Marker trait **not** implemented for tuple of the same types - `(T, T)`.
    ///
    /// If `(A, B): TypeNeq` then `A` != `B`
    pub auto trait TypeNeq {}
    impl<T> !TypeNeq for (T, T) {}
}
