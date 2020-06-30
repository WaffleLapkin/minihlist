use crate::{Cons, Nil, SmallHList};

pub trait Tuple {
    type HListRepr: SmallHList;

    fn from_hlist(hlist: Self::HListRepr) -> Self;

    fn into_hlist(self) -> Self::HListRepr;
}

impl Tuple for () {
    type HListRepr = Nil;

    fn from_hlist(_: Self::HListRepr) -> Self {}

    fn into_hlist(self) -> Self::HListRepr {
        Nil
    }
}

macro_rules! impl_tuple {
    ($first:ident, $( $types:ident, )*) => {
        impl<$first, $( $types, )*> Tuple for ($first, $( $types, )*) {
            type HListRepr = Cons<$first, <( $( $types, )* ) as Tuple>::HListRepr>;

            fn from_hlist(hlist: Self::HListRepr) -> Self {
                #[allow(non_snake_case)]
                let ( $( $types, )* ) = <( $( $types, )* ) as Tuple>::from_hlist(hlist.1);
                (hlist.0, $( $types ),*)
            }

            fn into_hlist(self) -> Self::HListRepr {
                #[allow(non_snake_case)]
                let ( $first, $( $types, )* ) = self;
                Cons($first, ( $( $types, )* ).into_hlist())
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, # impl_tuple);
