use crate::{pure, Cons, Nil};

/// This trait allows to Map elements of a `HList`.
///
/// Essentially this means applying a function to each element of a HList.
///
/// The function(s) type `F` can be:
/// 1. A `HList` of `FnOnce` functions (one for each element)
/// 2. A single `FnMut` function (for homogeneous `HList`s) (Note: due to some limitations you need
///    a mutable reference to `FnMut` - `&'_ mut impl FnMut`)
/// 3. Combination of 1 and 2: a `HList` of `FnOnce` functions last of which is `FnMut` function
///    (for homogeneous tails)
///
/// See examples for more.
///
/// Note: `hlist![v0, v1, ..., vn].map(hlist![f0, f1, ..., fn])` is essentially
/// `hlist![f0(v0), f1(v1), ..., fn(vn)]`
///
/// ## Examples
///
/// Basic usage:
///
/// ```
/// use minihlist::{hlist, Map};
/// let res = hlist![10u16, 3u32, 42u64].map(hlist![
///     |x| x + 2,
///     |y| y * 2,
///     |z| z - 2,
/// ]);
/// assert_eq!(res, hlist![12, 6, 40]);
/// ```
///
/// Using `FnMut` to map the homogenous list:
///
/// ```
/// use minihlist::{hlist, Map};
/// use std::ops::Neg;
///
/// let res = hlist![1, 2, 3].map(&mut Neg::neg);
/// assert_eq!(res, hlist![-1, -2, -3]);
/// ```
///
/// Using `FnMut` to map the homogenous tail:
///
/// ```
/// use minihlist::{hlist, Map};
/// let res = hlist![1.5, 12, 16].map(hlist![
///     |f| (f * 2.) as i32,
///     &mut |i| format!("{}", i), // <-- `FnMut` here
/// ]);
/// assert_eq!(res, hlist![3, "12", "16"]);
/// ```

pub trait Map<F> {
    type Output;

    fn map(self, f: F) -> Self::Output;
}

impl<FH, FM, FT, H, M, T, R> Map<Cons<FH, Cons<FM, FT>>> for Cons<H, Cons<M, T>>
where
    FH: FnOnce(H) -> R,
    Cons<M, T>: Map<Cons<FM, FT>>,
{
    #[allow(clippy::type_complexity)]
    type Output = Cons<R, <Cons<M, T> as Map<Cons<FM, FT>>>::Output>;

    fn map(self, Cons(fh, ft): Cons<FH, Cons<FM, FT>>) -> Self::Output {
        let Cons(head, tail) = self;
        Cons(fh(head), tail.map(ft))
    }
}

impl<F, H, R> Map<Cons<F, Nil>> for Cons<H, Nil>
where
    F: FnOnce(H) -> R,
{
    type Output = Cons<R, Nil>;

    fn map(self, Cons(f, _): Cons<F, Nil>) -> Self::Output {
        let Cons(head, _) = self;
        pure(f(head))
    }
}

// the reference (`&'a mut`) should *not* be required here but due to a bug (?) in rustc, without it
// the code doesn't compile. See https://github.com/rust-lang/rust/issues/74789
impl<'a, F, H, T, R> Map<&'a mut F> for Cons<H, T>
where
    F: FnMut(H) -> R,
    T: Map<&'a mut F>,
{
    type Output = Cons<R, T::Output>;

    fn map(self, f: &'a mut F) -> Self::Output {
        let Cons(head, tail) = self;
        Cons(f(head), tail.map(f))
    }
}

impl<F, H, M, T> Map<Cons<F, Nil>> for Cons<H, Cons<M, T>>
where
    Self: Map<F>,
{
    type Output = <Self as Map<F>>::Output;

    fn map(self, Cons(f, _): Cons<F, Nil>) -> Self::Output {
        self.map(f)
    }
}

impl<F> Map<F> for Nil {
    type Output = Nil;

    fn map(self, _: F) -> Self::Output {
        Nil
    }
}

#[test]
fn getero() {
    use crate::hlist;

    let list = hlist![1, 'x', 4u8];
    let f = hlist![|i| i + 1, |c: char| c.to_ascii_uppercase(), i64::from];

    assert_eq!(list.map(f), hlist![2, 'X', 4i64]);
}

#[test]
fn homo() {
    use crate::hlist;

    let list = hlist![1, 2, 3];
    let f = &mut |i| i64::from(i) + 1;

    assert_eq!(list.map(f), hlist![2i64, 3i64, 4i64]);
}

#[test]
fn mixed() {
    use crate::hlist;

    let list = hlist!['x', "x", 2u8, 4u8, 8u8];
    let f = hlist![
        |c: char| c.to_ascii_uppercase(),
        |s: &str| s.to_uppercase(),
        &mut |i| i * 2,
    ];

    assert_eq!(list.map(f), hlist!['X', "X", 4, 8, 16]);
}
