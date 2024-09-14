//! Wrapper around indexing for opt-in, additional safety.
//!
//! By default, writers tend to be safe, due to Miri, Valgrind,
//! and other tests and careful validation against a wide range
//! of randomized input. Parsers are much trickier to validate.

// `index_unchecked_mut`'s 2nd arm is unused in `compact`.
#![cfg_attr(feature = "compact", allow(unused_macros, unused_macro_rules))]
#![doc(hidden)]

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

/// Index a buffer and get a mutable reference, without bounds checking.
// The `($x:ident[$i:expr] = $y:ident[$j:expr])` is not used with `compact`.
// The newer version of the lint is `unused_macro_rules`, but this isn't
// supported until nightly-2022-05-12.
#[cfg(not(feature = "safe"))]
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! index_unchecked_mut {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked_mut($i)
    };

    ($x:ident[$i:expr] = $y:ident[$j:expr]) => {
        *$x.get_unchecked_mut($i) = *$y.get_unchecked($j)
    };
}

/// Fill a slice with a value, without bounds checking.
#[cfg(not(feature = "safe"))]
macro_rules! slice_fill_unchecked {
    ($slc:expr, $value:expr) => {
        // Get the length first to avoid stacked borrows, since we might
        // have a complex expression for the slice calculation.
        let len = $slc.len();
        core::ptr::write_bytes($slc.as_mut_ptr(), $value, len)
    };
}

/// Fill a slice with a value, with bounds checking.
#[cfg(feature = "safe")]
macro_rules! slice_fill_unchecked {
    ($slc:expr, $value:expr) => {
        $slc.fill($value)
    };
}

/// Index a buffer and get a mutable reference, with bounds checking.
// The `($x:ident[$i:expr] = $y:ident[$j:expr])` is not used with `compact`.
// The newer version of the lint is `unused_macro_rules`, but this isn't
// supported until nightly-2022-05-12.
#[cfg(feature = "safe")]
#[allow(unknown_lints, unused_macro_rules)]
macro_rules! index_unchecked_mut {
    ($x:ident[$i:expr]) => {
        $x[$i]
    };

    ($x:ident[$i:expr] = $y:ident[$j:expr]) => {
        $x[$i] = $y[$j]
    };
}
