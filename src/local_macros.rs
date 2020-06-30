/// Call macro for tuples
///
/// `tuple_impls!(C, B, A, ; cb)` will call `cb` with
/// - `cb!(A,)`
/// - `cb!(A, B)`
/// - `cb!(A, B, C)`
macro_rules! for_tuples {
    ( $( $types:ident, )* @ # $cb:ident) => {
        $cb!($( $types, )*);
    };
    ( $( $types:ident, )* @ $ty:ident, $( $rest:ident, )* # $cb:ident) => {
        $cb!($( $types, )*);
        for_tuples!($( $types, )* $ty, @ $( $rest, )* # $cb);
    };
    ( $ty:ident, $( $rest:ident, )* # $cb:ident) => {
        for_tuples!( $ty, @ $( $rest, )* # $cb);
    };
    () => {};
}
