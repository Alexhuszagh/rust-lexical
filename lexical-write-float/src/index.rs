//! Wrapper around indexing for opt-in, additional safety.
//!
//! By default, writers tend to be safe, due to Miri, Valgrind,
//! and other tests and careful validation against a wide range
//! of randomized input. Parsers are much trickier to validate.

#![cfg_attr(not(feature = "power-of-two"), allow(unused_macros))]

/// Index a buffer, without bounds checking.
#[cfg(not(feature = "safe"))]
macro_rules! index_unchecked {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked($i)
    };
}

/// Index a buffer and get a mutable reference, without bounds checking.
#[cfg(not(feature = "safe"))]
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
        core::ptr::write_bytes($slc.as_mut_ptr(), $value, $slc.len())
    };
}

/// Copy to a slice, without bounds checking.
#[cfg(not(feature = "safe"))]
macro_rules! copy_unchecked {
    ($dst:expr, $src:expr, $srclen:expr) => {
        core::ptr::copy($src, $dst.as_mut_ptr(), $srclen)
    };
}

/// Copy to a slice without overlaps, without bounds checking.
#[cfg(not(feature = "safe"))]
macro_rules! copy_nonoverlapping_unchecked {
    ($dst:expr, $src:expr, $srclen:expr) => {
        core::ptr::copy_nonoverlapping($src, $dst.as_mut_ptr(), $srclen)
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
macro_rules! index_unchecked_mut {
    ($x:ident[$i:expr]) => {
        $x[$i]
    };

    ($x:ident[$i:expr] = $y:ident[$j:expr]) => {
        $x[$i] = $y[$j]
    };
}

/// Fill a slice with a value, with bounds checking.
#[cfg(feature = "safe")]
macro_rules! slice_fill_unchecked {
    ($slc:expr, $value:expr) => {
        $slc.fill($value)
    };
}

/// Copy to a slice, with bounds checking.
#[cfg(feature = "safe")]
macro_rules! copy_unchecked {
    ($dst:expr, $src:expr, $srclen:expr) => {
        $dst.copy_from_slice(unsafe { core::slice::from_raw_parts($src, $srclen) })
    };
}

/// Copy to a slice, with bounds checking.
#[cfg(feature = "safe")]
macro_rules! copy_nonoverlapping_unchecked {
    ($dst:expr, $src:expr, $srclen:expr) => {
        copy_unchecked!($dst, $src, $srclen)
    };
}
