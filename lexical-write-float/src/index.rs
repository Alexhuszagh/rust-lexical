//! Wrapper around indexing for opt-in, additional safety.
//!
//! By default, writers tend to be safe, due to Miri, Valgrind,
//! and other tests and careful validation against a wide range
//! of randomized input. Parsers are much trickier to validate.

#![cfg_attr(feature = "compact", allow(unused_macros))]
#![doc(hidden)]

/// Index a buffer, without bounds checking.
macro_rules! index_unchecked {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked($i)
    };
}
