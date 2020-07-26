use crate::{Cons, Nil};

/// Right fold over a HList.
///
/// This applies function(s) to all elements in the right-to-left order accumulating and returning
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
/// `f0(f1(... fn(acc, vn) ..., v1), v0)` or
/// ```ignore
/// let acc = fn(acc, vn)
/// // ...
/// let acc = f1(acc, v1);
/// f0(acc, v0)
/// ```
///
/// ## Examples
///
/// Basic usage:
///
/// ```
/// use minihlist::{hlist, FoldRight};
/// let res = hlist![b' ', 13, false].rfold(1, hlist![
///     |acc, c: u8| acc + (c as i32),
///     |acc, i| acc * i,
///     |acc, b| if b { 10 } else { acc },
/// ]);
/// assert_eq!(res, 45);
/// ```
///
/// Using `FnMut` to fold the homogenous list:
///
/// ```
/// use minihlist::{hlist, FoldRight};
/// use std::ops::{Add, Sub};
///
/// let sum = hlist![1, 2, 3].rfold(0, Add::add);
/// assert_eq!(sum, 6);
///
/// let res = hlist![1, 2, 3].rfold(String::from("0"), |acc, i| format!("({} + {})", acc, i));
/// assert_eq!(res, "(((0 + 3) + 2) + 1)");
/// ```
///
/// Using `FnMut` to fold the homogenous tail:
///
/// ```
/// use minihlist::{hlist, FoldRight};
/// let res = hlist![1.5, 12, 16].rfold(2, hlist![
///     |acc, f| (acc as f64 * f) as i32,
///     |acc, i| acc + i, // <-- `FnMut` here
/// ]);
/// assert_eq!(res, 45);
/// ```
pub trait FoldRight<Acc, F> {
    type Output;

    fn rfold(self, acc: Acc, f: F) -> Self::Output;
}

// Acc      - accumulator,
// F{H,M,T} - function {head,middle,tail} parts
// {H,M,T}  - {head,middle,tail} of the hlist being folded
// R        - result of `FH`
impl<Acc, H, M, T, FH, FM, FT, R> FoldRight<Acc, Cons<FH, Cons<FM, FT>>> for Cons<H, Cons<M, T>>
where
    Cons<M, T>: FoldRight<Acc, Cons<FM, FT>>,
    FH: FnOnce(<Cons<M, T> as FoldRight<Acc, Cons<FM, FT>>>::Output, H) -> R,
{
    type Output = R;

    #[inline]
    fn rfold(self, acc: Acc, Cons(fh, ft): Cons<FH, Cons<FM, FT>>) -> Self::Output {
        let Cons(head, tail) = self;
        fh(tail.rfold(acc, ft), head)
    }
}

impl<Acc, H, F, R> FoldRight<Acc, Cons<F, Nil>> for Cons<H, Nil>
where
    F: FnOnce(Acc, H) -> R,
{
    type Output = R;

    #[inline]
    fn rfold(self, acc: Acc, Cons(f, _): Cons<F, Nil>) -> Self::Output {
        let Cons(head, _) = self;
        f(acc, head)
    }
}

impl<Acc, H, T, F> FoldRight<Acc, F> for Cons<H, T>
where
    for<'a> T: FoldRight<Acc, &'a mut F, Output = Acc>,
    F: FnMut(Acc, H) -> Acc,
{
    type Output = Acc;

    #[inline]
    fn rfold(self, acc: Acc, mut f: F) -> Self::Output {
        let Cons(head, tail) = self;
        let acc = tail.rfold(acc, &mut f);
        f(acc, head)
    }
}

impl<Acc, H, T0, T1, F> FoldRight<Acc, Cons<F, Nil>> for Cons<H, Cons<T0, T1>>
where
    Self: FoldRight<Acc, F>,
{
    type Output = <Self as FoldRight<Acc, F>>::Output;

    #[inline]
    fn rfold(self, acc: Acc, Cons(f, _): Cons<F, Nil>) -> Self::Output {
        self.rfold(acc, f)
    }
}

impl<Acc, F> FoldRight<Acc, F> for Nil {
    type Output = Acc;

    #[inline]
    fn rfold(self, acc: Acc, _: F) -> Self::Output {
        acc
    }
}
