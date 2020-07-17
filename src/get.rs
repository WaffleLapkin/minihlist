use crate::{
    succnum::{Succ, Zero},
    Cons,
};

/// Type-directed search in hlist.
///
/// This trait allows to borrow elements from a hlist just by type.
///
/// ## Examples
///
/// ```
/// use minihlist::{hlist, Get};
///
/// let mut list = hlist![1, 'x'];
///
/// let i: &i32 = list.get();
/// assert_eq!(i, &1);
///
/// *list.get_mut() = 'y';
/// assert_eq!(list, hlist![1, 'y']);
/// ```
///
/// Note that you can't borrow elements those are not unique:
/// ```compile_fail,E0282
/// use minihlist::{hlist, Get};
///
/// let list = hlist![17, 42];
/// let _: &i32 = list.get();
/// ```
pub trait Get<Idx, Out> {
    fn get(&self) -> &Out;

    fn get_mut(&mut self) -> &mut Out;
}

impl<H, T> Get<Zero, H> for Cons<H, T> {
    fn get(&self) -> &H {
        &self.0
    }

    fn get_mut(&mut self) -> &mut H {
        &mut self.0
    }
}

impl<H, T, Idx, Out> Get<Succ<Idx>, Out> for Cons<H, T>
where
    T: Get<Idx, Out>,
{
    fn get(&self) -> &Out {
        self.1.get()
    }

    fn get_mut(&mut self) -> &mut Out {
        self.1.get_mut()
    }
}
