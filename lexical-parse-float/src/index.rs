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

/// Index a buffer, with bounds checking.
#[cfg(feature = "safe")]
macro_rules! index_unchecked {
    ($x:ident[$i:expr]) => {
        $x[$i]
    };
}
