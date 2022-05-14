//! Wrapper around indexing for opt-in, additional safety.
//!
//! This is used very sparing in parsers, only when code can trivially
//! be shown to be safe, since parsers are tricky to validate.

#![cfg_attr(not(feature = "power-of-two"), allow(unused_macros))]
#![doc(hidden)]

/// Index a buffer, without bounds checking.
#[cfg(not(feature = "safe"))]
macro_rules! index_unchecked {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked($i)
    };
}

/// Index a buffer and get a mutable reference, without bounds checking.
#[cfg(not(feature = "safe"))]
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! index_unchecked_mut {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked_mut($i)
    };
}

/// Index a buffer, with bounds checking.
#[cfg(feature = "safe")]
macro_rules! index_unchecked {
    ($x:ident[$i:expr]) => {
        $x[$i]
    };
}

/// Index a buffer and get a mutable reference, with bounds checking.
#[cfg(feature = "safe")]
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! index_unchecked_mut {
    ($x:ident[$i:expr]) => {
        $x[$i]
    };
}
