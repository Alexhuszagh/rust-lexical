//! C++-like range adaptors.

use super::algorithm::*;

// RANGE

/// C++-style range adaptor with helper methods.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    /// First element in range.
    pub first: *const u8,
    /// Last element in range.
    pub last: *const u8,
}

// Allow dead code, since we have helper functions that are used on some
// configurations, and not on others. Their functionality is clean, allow
// some dead code.
#[allow(dead_code)]
impl Range {
    /// Create new range.
    #[inline(always)]
    pub unsafe extern "C" fn new(first: *const u8, last: *const u8)
        -> Range
    {
        Range {
            first: first,
            last: last,
        }
    }

    /// Calculate the difference between two first and last.
    #[inline(always)]
    pub unsafe extern "C" fn distance(&self)
        -> usize
    {
        distance(self.first, self.last)
    }

    /// Check if two ranges are equal to each other.
    #[inline(always)]
    pub unsafe extern "C" fn equal_to(&self, y: &Range, n: usize)
        -> bool
    {
        equal_to_range(self.first, y.first, n)
    }

    /// Check if two ranges are equal to each other without case-sensitivity.
    #[inline(always)]
    pub unsafe extern "C" fn case_insensitive_equal_to(&self, y: &Range, n: usize)
        -> bool
    {
        case_insensitive_equal_to_range(self.first, y.first, n)
    }

    /// Check if left range starts with right range.
    #[inline(always)]
    pub unsafe extern "C" fn starts_with(&self, y: &Range)
        -> bool
    {
        starts_with_range(self.first, self.distance(), y.first, y.distance())
    }

    /// Check if left range starts with right range without case-sensitivity.
    #[inline(always)]
    pub unsafe extern "C" fn case_insensitive_starts_with(&self, y: &Range)
        -> bool
    {
        case_insensitive_starts_with_range(self.first, self.distance(), y.first, y.distance())
    }

    /// Check if left range ends with right range.
    #[inline(always)]
    pub unsafe extern "C" fn ends_with(&self, y: &Range)
        -> bool
    {
        ends_with_range(self.first, self.distance(), y.first, y.distance())
    }

    /// Check if left range ends with right range without case-sensitivity.
    #[inline(always)]
    pub unsafe extern "C" fn case_insensitive_ends_with(&self, y: &Range)
        -> bool
    {
        case_insensitive_ends_with_range(self.first, self.distance(), y.first, y.distance())
    }

    /// Trim character from the left-side of a range.
    #[inline(always)]
    pub unsafe extern "C" fn ltrim_char(&mut self, c: u8)
    {
        self.first = ltrim_char_range(self.first, self.last, c)
    }

    /// Trim character from the left-side of a range without case-sensitivity.
    #[inline(always)]
    pub unsafe extern "C" fn case_insensitive_ltrim_char(&mut self, c: u8)
    {
        self.first = case_insensitive_ltrim_char_range(self.first, self.last, c)
    }
}
