use crate::{Nil, Cons, Exclude};

/// Marker trait that is implemented for `HList`s those have no repeating types.
///
/// ## Examples
///
/// ```
/// use minihlist::{HList, Unique, Nil};
/// fn assert_ty<L: Unique>() {}
///
/// assert_ty::<HList![usize, String, char]>();
/// assert_ty::<HList![u32, i32]>();
/// ```
///
/// ```compile_fail,E0277
/// use minihlist::{HList, Unique};
/// fn assert_ty<L: Unique>() {}
///
/// // fails to compile: `i32` is repeating 2 times
/// assert_ty::<HList![i32, usize, i32]>();
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
pub trait Unique {}

impl Unique for Nil {}

impl<H, T> Unique for Cons<H, T>
where
    T: Exclude<H> + Unique,
{}
