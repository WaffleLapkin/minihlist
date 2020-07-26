use crate::{Cons, Nil};

/// Left fold over a HList.
///
/// This applies function(s) to all elements in the left-to-right order accumulating and returning
/// a value.
///
/// The function(s) type `F` can be:
/// 1. A `HList` of `FnOnce` functions (one for each element)
/// 2. A single `FnMut` functions (for homogenous `HList`s)
/// 3. Combination of 1 and 2: a `HList` of `FnOnce` functions last of which is `FnMut` function
///    (for homogenous tails)
///
/// See examples for more.
///
/// The accumulator type can change from function to function.
///
/// Note: `hlist![v0, v1, ..., vn].fold(acc, hlist![f0, f1, ..., fn])` is equivalent to
/// `fn( ... f1(f0(acc, v0), v1) ..., vn)` or
/// ```ignore
/// let acc = f0(acc, v0);
/// let acc = f1(acc, v1);
/// // ...
/// fn(acc, vn)
/// ```
///
/// ## Examples
///
/// Basic usage:
///
/// ```
/// use minihlist::{hlist, Fold};
/// let res = hlist![10u16, 3u32, 42u64].fold(8u8, hlist![
///     |acc, a| acc as u16 + a,
///     |acc, b| acc as u32 * b,
///     |acc, c| acc as u64 - c,
/// ]);
/// assert_eq!(res, 12);
/// ```
///
/// Using `FnMut` to fold the homogenous list:
///
/// ```
/// use minihlist::{hlist, Fold};
/// use std::ops::Add;
///
/// let sum = hlist![1, 2, 3].fold(0, Add::add);
/// assert_eq!(sum, 6);
///
/// let res = hlist![1, 2, 3].fold(String::from("0"), |acc, i| format!("({} + {})", acc, i));
/// assert_eq!(res, "(((0 + 1) + 2) + 3)");
/// ```
///
/// Using `FnMut` to fold the homogenous tail:
///
/// ```
/// use minihlist::{hlist, Fold};
/// let res = hlist![1.5, 12, 16].fold(2., hlist![
///     |acc, f| (acc * f) as i32,
///     |acc, i| acc + i, // <-- `FnMut` here
/// ]);
/// assert_eq!(res, 31);
/// ```
pub trait Fold<Acc, F> {
    type Output;

    fn fold(self, acc: Acc, f: F) -> Self::Output;
}

// Acc      - accumulator,
// F{H,M,T} - function {head,middle,tail} parts
// {H,M,T}  - {head,middle,tail} of the hlist being folded
// R        - result of `FH`
impl<Acc, FH, FM, FT, H, M, T, R> Fold<Acc, Cons<FH, Cons<FM, FT>>> for Cons<H, Cons<M, T>>
where
    FH: FnOnce(Acc, H) -> R,
    Cons<M, T>: Fold<R, Cons<FM, FT>>,
{
    type Output = <Cons<M, T> as Fold<R, Cons<FM, FT>>>::Output;

    #[inline]
    fn fold(self, acc: Acc, Cons(fh, ft): Cons<FH, Cons<FM, FT>>) -> Self::Output {
        let Cons(head, tail) = self;
        tail.fold(fh(acc, head), ft)
    }
}

impl<Acc, H, F, R> Fold<Acc, Cons<F, Nil>> for Cons<H, Nil>
where
    F: FnOnce(Acc, H) -> R,
{
    type Output = R;

    #[inline]
    fn fold(self, acc: Acc, Cons(f, _): Cons<F, Nil>) -> Self::Output {
        let Cons(head, _) = self;
        f(acc, head)
    }
}

impl<Acc, H, T, F> Fold<Acc, F> for Cons<H, T>
where
    F: FnMut(Acc, H) -> Acc,
    T: Fold<Acc, F>,
{
    type Output = T::Output;

    #[inline]
    fn fold(self, acc: Acc, mut f: F) -> Self::Output {
        let Cons(head, tail) = self;
        tail.fold(f(acc, head), f)
    }
}

impl<Acc, H, T0, T1, F> Fold<Acc, Cons<F, Nil>> for Cons<H, Cons<T0, T1>>
where
    Self: Fold<Acc, F>,
{
    type Output = <Self as Fold<Acc, F>>::Output;

    #[inline]
    fn fold(self, acc: Acc, Cons(f, _): Cons<F, Nil>) -> Self::Output {
        self.fold(acc, f)
    }
}

impl<Acc, F> Fold<Acc, F> for Nil {
    type Output = Acc;

    #[inline]
    fn fold(self, acc: Acc, _: F) -> Self::Output {
        acc
    }
}
