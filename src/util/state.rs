//! Internal state for the integer and float parsers.

use lib::{mem, ptr};
use super::algorithm::*;

// PARSE STATE

/// State for integer or float parser.
///
/// `trunc` when not set is set to null, otherwise, the number of truncated
/// bytes is equal to `distance(trunc, curr)`. Any other call will affect
/// this, so it is to be used sparingly.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseState {
    /// Current position in the buffer.
    pub curr: *const u8,
    /// Position where the integer was truncated or overflowed.
    pub trunc: *const u8,
}

// Allow dead code, since we have helper functions that are used on some
// configurations, and not on others. Their functionality is clean, allow
// some dead code.
#[allow(dead_code)]
impl ParseState {
    // CONSTRUCTORS

    /// Create new state.
    #[inline(always)]
    pub unsafe extern "C" fn new(p: *const u8)
        -> ParseState
    {
        ParseState {
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
        //debug_assert!(!self.trunc.is_null(), "ParseState::is_truncated() set_default_trunc() not called.");
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
