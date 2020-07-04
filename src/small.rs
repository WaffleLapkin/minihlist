use crate::{HList, Nil, Tuple};

pub trait SmallHList: HList {
    type TupleRepr: Tuple;

    fn into_tuple(self) -> Self::TupleRepr;

    fn from_tuple(repr: Self::TupleRepr) -> Self;
}

impl SmallHList for Nil {
    type TupleRepr = ();

    fn into_tuple(self) -> Self::TupleRepr {}

    fn from_tuple(_repr: Self::TupleRepr) -> Self {
        Nil
    }
}

macro_rules! impl_smallhlist {
    ($( $types:ident, )+) => {
        impl<$( $types, )*> SmallHList for $crate::HList![$( $types, )*] {
            type TupleRepr = ($( $types, )*);

            fn from_tuple(repr: Self::TupleRepr) -> Self {
                repr.into_hlist()
            }

            fn into_tuple(self) -> Self::TupleRepr {
                Self::TupleRepr::from_hlist(self)
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, # impl_smallhlist);
