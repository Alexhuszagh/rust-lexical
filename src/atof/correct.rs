//! Moderately fast, correct base10 lexical string-to-float conversion routines.

use super::algorithm::correct;

// TODO(ahuszagh)
//  Base implementation off of:
// Use fast path or bigint as a fallback.
//      https://github.com/gcc-mirror/gcc/blob/master/libgo/go/strconv/atof.go
//      https://github.com/python/cpython/blob/e42b705188271da108de42b55d9344642170aa2b/Python/dtoa.c

//

// F32

/// Import float from base10, using a correct algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[allow(unused)]        // TODO(ahuszagh) Implement...
pub(crate) unsafe extern "C" fn float_base10(first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    unreachable!()
}

// F64

/// Import double from base10, using a correct algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[allow(unused)]        // TODO(ahuszagh) Implement...
pub(crate) unsafe extern "C" fn double_base10(first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    unreachable!()
}
