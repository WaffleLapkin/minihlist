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
    private::Pair<H, E>: private::TypeNeq,
    T: Exclude<E>,
{
}

mod private {
    /// `Pair` is being used instead of `(_, _)` to prevent false negative errors with types those
    /// include `(T, T)` themselves (e.g.: `(i32, i32)`). See [#4] & test `hlist_with_tuple2` down
    /// below for more.
    ///
    /// [#4]: https://github.com/WaffleLapkin/minihlist/issues/4
    pub struct Pair<A, B>(A, B);

    /// Marker trait **not** implemented for pair of the same types - `Pair<T, T>`.
    ///
    /// If `Pair<A, B>: TypeNeq` then `A` != `B`
    pub auto trait TypeNeq {}
    impl<T> !TypeNeq for Pair<T, T> {}
}

/// Test for issue [#4](https://github.com/WaffleLapkin/minihlist/issues/4)
#[test]
fn hlist_with_tuple2() {
    fn assert<T: Exclude<E>, E>() {}

    type DefinitelyNotATuple = u64;
    assert::<crate::HList![(i32, i32)], DefinitelyNotATuple>();
}
