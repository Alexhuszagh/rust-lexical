//! Internal state for the integer and float parsers.

use lib::{mem, ptr};
use super::algorithm::*;
use super::range::*;

// PARSE INT STATE

/// State for the integer parser.
///
/// `trunc`, during integer parsing, is tentatively set to `null`, however,
/// it should be set to `curr` or the truncated position during parsing.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseIntState {
    /// Current position in the buffer.
    pub curr: *const u8,
    /// Position where the integer was truncated or overflowed.
    pub trunc: *const u8,
}

// Allow dead code, since we have helper functions that are used on some
// configurations, and not on others. Their functionality is clean, allow
// some dead code.
#[allow(dead_code)]
impl ParseIntState {
    // CONSTRUCTORS

    /// Create new state.
    #[inline(always)]
    pub unsafe extern "C" fn new(p: *const u8)
        -> ParseIntState
    {
        ParseIntState {
            curr: p,
            trunc: ptr::null()
        }
    }

    // PROPERTIES

    /// Get the number of bytes truncated during parsing.
    #[inline(always)]
    pub unsafe extern "C" fn truncated_bytes(&self)
        -> usize
    {
        //debug_assert!(!self.trunc.is_null(), "ParseIntState::is_truncated() set_default_trunc() not called.");
        if self.is_truncated() {
            distance(self.trunc, self.curr)
        } else {
            0
        }
    }

    /// Determine if the integer parsed truncated or overflowed.
    #[inline(always)]
    pub unsafe extern "C" fn is_truncated(&self) -> bool {
        !self.trunc.is_null()
    }

    // MODIFIERS

    /// Increment curr by 1.
    #[inline(always)]
    pub unsafe extern "C" fn increment(&mut self) {
        self.curr = self.curr.add(1);
    }

    /// Set default truncated (curr) if not set.
    #[inline(always)]
    pub unsafe extern "C" fn set_default_trunc(&mut self) {
        if self.trunc.is_null() {
            self.trunc = self.curr;
        }
    }

    /// Trim character from the left-side of a range.
    #[inline(always)]
    pub unsafe extern "C" fn ltrim_char(&mut self, last: *const u8, c: u8) {
        self.curr = ltrim_char_range(self.curr, last, c);
    }
}

// PARSE FLOAT STATE

/// State for the float parser.
///
/// Disable lifetime checks, since we guarantee the pointers
/// will be valid over the entire lifetime of ParseFloatState.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseFloatState {
    /// Inner state
    pub inner: ParseIntState,
    /// Mantissa substring of float.
    pub mant: Range,
    /// Exponent substring of float.
    pub exp: Range,
}

impl ParseFloatState {
    /// Create new state.
    #[inline(always)]
    pub unsafe extern "C" fn new(p: *const u8)
        -> ParseFloatState
    {
        ParseFloatState {
            inner: ParseIntState::new(p),
            mant: mem::uninitialized(),
            exp: mem::uninitialized(),
        }
    }

    // PROPERTIES

    /// Get the number of bytes truncated during parsing.
    ///
    /// This may over-estimate the number of truncated bytes by 1, since
    /// if the
    #[inline(always)]
    pub unsafe extern "C" fn truncated_bytes(&self)
        -> usize
    {
        if self.is_truncated() {
            distance(self.inner.trunc, self.mant.last)
        } else {
            0
        }
    }

    /// Determine if the integer parsed truncated or overflowed.
    #[inline(always)]
    pub unsafe extern "C" fn is_truncated(&self) -> bool {
        self.inner.is_truncated()
    }
}
