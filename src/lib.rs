//! This crate provides [`Cons`] and [`Nil`] structs, which in couple form so-called
//! "heterogeneous list" (`HList`), as well as traits/functions to work with them.
//!
//! `HList` has a lot of common with [tuples], however it also has a lot of
//! advantages (and some disadvantages).
//!
//! [`Cons`] in a simple tuple-like struct with 2 generics and 2 fields of those types. [`Nil`]
//! is even simpler - it's just a unit struct.
//!
//! ```
//! // real code from the `minihlist`
//! // (but with some boilerplate like derives lifted out)
//! pub struct Cons<H, T>(pub H, pub T);
//! pub struct Nil;
//! ```
//!
//! In most simple cases `HList` can act just like [tuples]:
//!
//! ```
//! use minihlist::{hlist, hpat, HList};
//!
//! // create a hlist with 3 elements
//! let list = hlist!["hello", 5, 'c'];
//!
//! // destruct the list back
//! let hpat![a, b, c] = list;
//! assert_eq!(a, "hello");
//! assert_eq!(b, 5);
//! assert_eq!(c, 'c');
//!
//! // use in functions
//! fun(list);
//! fn fun(list: HList![&'static str, i32, char]) {
//!     const C: char = 'c';
//!     match list {
//!         // pattern matching
//!         hpat!["hello", x @ 5, C] => {},
//!         _ => unreachable!()
//!     }
//!
//!     //assert_eq!(format!("{:#?}", list), "(\"hello\"), 5, 'c'"); // TODO
//! }
//! ```
//!
//! [tuples]: https://doc.rust-lang.org/std/primitive.tuple.html
//!
//!
//!
//! TODO
#![cfg_attr(feature = "nightly", feature(optin_builtin_traits, negative_impls))]
// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs" cargo doc --open --all-features
// ```
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg))]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

#[macro_use]
/// Helper macros these are used in this lib
mod local_macros;

mod append;
mod extend;
mod fold;
mod get;
mod hlist;
mod map;
mod remove;
mod rev;
mod rfold;
mod small;
mod tuple;

#[cfg(feature = "nightly")]
mod flatten;

#[cfg(feature = "typenum")]
mod len;

pub use self::{
    append::Append, extend::Extend, fold::Fold, get::Get, hlist::HList, map::Map, remove::Remove, rev::Rev,
    rfold::FoldRight, small::SmallHList, tuple::Tuple,
};

#[cfg(feature = "typenum")]
pub use len::Len;

#[cfg(feature = "nightly")]
pub use self::flatten::Flatten;

/// The empty `HList`.
///
/// See [crate documentation](./index.html) for more.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Nil;

/// An `HList` with `H` at position 0, and `T` as the rest of the list.
///
/// See [crate documentation](./index.html) for more.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cons<H, T>(pub H, pub T);

/// Creates a HList with a single element `head` in it.
///
/// ```
/// use minihlist::{Cons, Nil, hlist, HList};
///
/// assert_eq!(minihlist::pure(10), Cons(10, Nil));
/// assert_eq!(minihlist::pure(Some(false)), hlist![Some(false)]);
/// assert_eq!(minihlist::pure("hi").push(true), Cons(true, Cons("hi", Nil)));
/// ```
pub const fn pure<T>(head: T) -> Cons<T, Nil> {
    Cons(head, Nil)
}

impl From<()> for Nil {
    fn from(_: ()) -> Self {
        Self
    }
}

impl<H> From<H> for Cons<H, Nil> {
    fn from(head: H) -> Self {
        pure(head)
    }
}

impl<H, T> From<(H, T)> for Cons<H, T> {
    fn from((head, tail): (H, T)) -> Self {
        Self(head, tail)
    }
}

impl<H, T> Cons<H, T> {
    /// Pops the head of the list, returning tuple of the head and the tail.
    ///
    /// ```
    /// use minihlist::{hlist, hpat, Cons};
    ///
    /// let (head, tail) = hlist![1, false, ()].pop();
    /// assert_eq!(head, 1);
    /// assert_eq!(tail, hlist![false, ()]);
    /// // analog to any of
    /// let Cons(_head, _tail) = hlist![1, false, ()];
    /// let hpat![_head, _tail @ ..] = hlist![1, false, ()];
    /// ```
    pub fn pop(self) -> (H, T) {
        let Self(head, tail) = self;
        (head, tail)
    }
}

/// ## Examples
///
/// Basic usage:
///
/// ```
/// use minihlist::{Cons, Nil, hlist};
/// assert_eq!(
///     hlist![1, "hi", Some(3)],
///     Cons(1, Cons("hi", Cons(Some(3), Nil))),
/// );
/// ```
///
/// Use `...list` to flatten hlist:
///
/// ```
/// use minihlist::{Cons, Nil, hlist};
/// let list = hlist!["x", "y"];
///
/// // flatten
/// assert_eq!(
///     hlist![1, ...list, 42],
///     Cons(1, Cons("x", Cons("y", Cons(42, Nil)))),
/// );
///
/// // list as element
/// assert_eq!(
///     hlist![1, list, 42],
///     Cons(1, Cons(Cons("x", Cons("y", Nil)), Cons(42, Nil))),
/// );
///
/// // you can flatten many lists
/// assert_eq!(
///     hlist![1, ...list, ...hlist![1..12, 8usize]],
///     Cons(1, Cons("x", Cons("y", Cons(1..12, Cons(8, Nil))))),
/// );
/// ```
#[macro_export]
macro_rules! hlist {
    (...$head:expr, $( $tail:tt )*) => {
        $crate::Extend::extend($head, $crate::hlist![ $( $tail )* ])
    };
    (...$head:expr) => { $crate::hlist![...$head,] /* redirect to previous branch */ };
    ($head:expr, $( $tail:tt )*) => {
        $crate::Cons {
            0: $head,
            1: $crate::hlist![ $( $tail )* ],
        }
    };
    ($head:expr) => { $crate::pure($head)  };
    () => { $crate::Nil };
}

/// ## Examples
///
/// Basic usage:
///
/// ```
/// use minihlist::{Cons, Nil, hlist, hpat};
/// let list = hlist!["x", "y"];
///
/// let hpat![a, b, c] = hlist![17, "h", ' '];
/// assert_eq!(a, 17);
/// assert_eq!(b, "h");
/// assert_eq!(c, ' ');
///
/// match hlist![1, "2", 'x'] {
///     hpat![1, _, x] => {}
///     _ => unreachable!(),
/// }
/// ```
///
/// It's a compile error to not match all elements:
///
/// ```compile_fail,E0308
/// use minihlist::{hpat, hlist};
/// let hpat![a, b] = hlist![1, 2, 3];
/// ```
///
/// Use `..` to ignore the rest of elements or `rest @ ..` to match tail as hlist:
///
/// ```
/// use minihlist::{Cons, Nil, hlist, hpat};
///
/// let hpat![x, ..] = hlist!["x", "y", "z"];
/// assert_eq!(x, "x");
///
/// let hpat![a, rest @ ..] = hlist![17, "h", ' '];
/// assert_eq!(a, 17);
/// assert_eq!(rest, hlist!["h", ' ']);
/// ```
#[macro_export]
macro_rules! hpat {
    () => { $crate::Nil };
    (.. $(,)?) => { _ };
    ($rest:ident @ .. $(,)?) => { $rest };
    ($head:pat $( , $( $tail:tt )* )?) => {
        $crate::Cons {
            0: $head,
            1: $crate::hpat![ $( $( $tail )* )?],
        }
    };
}

/// ## Examples
///
/// Basic usage:
///
/// ```
/// use minihlist::{HList, hlist};
///
/// let _: HList![usize, (i32, i32)] = hlist![0, (18, 19)];
///
/// fn test(val: HList![i32, &str]) { /* ... */ }
/// test(hlist![1, "hi"]);
/// ```
///
/// Use `...list` to flatten hlist type:
///
/// ```
/// use minihlist::{HList, hlist};
///
/// let _: HList![&'static str, ...HList![i32, char], usize, ...HList![String]] =
///     hlist!["hi", 0, '1', 19, String::from("str ing")];
/// ```
#[macro_export]
macro_rules! HList {
    (...$head:ty, $( $tail:tt )*) => {
        <$head as $crate::Extend<$crate::HList![ $( $tail )* ]>>::Output
    };
    (...$head:ty) => { $crate::HList![...$head,] /* redirect to previous branch */ };
    ($head:ty, $( $tail:tt )*) => {
        $crate::Cons<$head, $crate::HList![ $( $tail )* ]>
    };
    ($head:ty) => { $crate::HList![$head,] /* redirect to previous branch */ };
    () => { $crate::Nil };
}

/// Minimalistic analog to crates like `peano` and `typenum`
mod succnum {
    pub enum Zero {}
    pub struct Succ<I>(I);
}
