//! An iterator that skips values equal to a provided value.
//!
//! SkipValueIterator iterates over a slice, returning all values
//! except for those matching the provided skip value.
//!
//! Example
//! -------
//!
//! ```text
//! let iter = SkipValueIterator(&[1, 2, 5, 2, 6, 7], 2);
//! assert!(iter.eq([1, 5, 6, 7].iter()));
//! ```

#![cfg(all(feature = "format", feature = "parse"))]

use crate::iterator::Iterator;
use crate::lib::{iter, slice};

// NEXT
// ----

/// Determine if the value is an internal digit separator.
#[inline]
fn is_i<'a, T, const FORMAT: u128>(value: &T, iter: &mut SkipValueIterator<'a, T, FORMAT>) -> bool
where
    T: 'a + PartialEq + Clone,
{
    // Need to check the value is not internal or trailing
    iter.has_started && iter.iter.slice_len() != 0 && value == &iter.skip
}

/// Determine if the value is a leading digit separator.
#[inline]
fn is_l<'a, T, const FORMAT: u128>(value: &T, iter: &mut SkipValueIterator<'a, T, FORMAT>) -> bool
where
    T: 'a + PartialEq + Clone,
{
    !iter.has_started && value == &iter.skip
}

/// Determine if the value is a trailing digit separator.
#[inline]
fn is_t<'a, T, const FORMAT: u128>(value: &T, iter: &mut SkipValueIterator<'a, T, FORMAT>) -> bool
where
    T: 'a + PartialEq + Clone,
{
    iter.iter.slice_len() == 0 && value == &iter.skip
}

/// Determine if the value is not an internal digit separator.
#[inline]
fn not_i<'a, T, const FORMAT: u128>(value: &T, iter: &mut SkipValueIterator<'a, T, FORMAT>) -> bool
where
    T: 'a + PartialEq + Clone,
{
    if value == &iter.skip {
        !iter.has_started || iter.iter.slice_len() == 0
    } else {
        true
    }
}

/// Determine if the value is not an leading digit separator.
#[inline]
fn not_l<'a, T, const FORMAT: u128>(value: &T, iter: &mut SkipValueIterator<'a, T, FORMAT>) -> bool
where
    T: 'a + PartialEq + Clone,
{
    iter.has_started || value != &iter.skip
}

/// Determine if the value is not a trailing digit separator.
#[inline]
fn not_t<'a, T, const FORMAT: u128>(value: &T, iter: &mut SkipValueIterator<'a, T, FORMAT>) -> bool
where
    T: 'a + PartialEq + Clone,
{
    iter.iter.slice_len() != 0 || value != &iter.skip
}

// These consume 1 or more valid digit separators and produce the next
// value from the input data.

/// Consumes internal digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_i<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // Skips a single internal, but not leading or trailing, digit separator.
    // This means if the consecutive item is a digit separator, then we yield
    // that value.
    let value = iter.iter.next()?;
    let skip = is_i(value, iter);
    iter.has_started = true;
    if skip {
        iter.next()
    } else {
        // Might be a digit separator, might not be: either way, we can't skip it.
        Some(value)
    }
}

/// Consumes leading digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_l<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This will only consume a single digit separator at the start of the slice.
    let value = iter.iter.next()?;
    let skip = is_l(value, iter);
    iter.has_started = true;
    if skip {
        iter.next()
    } else {
        // Might be a digit separator, might not be: either way, we can't skip it.
        Some(value)
    }
}

/// Consumes trailing digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_t<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This will only consume a single digit separator at the end of the slice.
    let value = iter.iter.next()?;
    if is_t(value, iter) {
        // Skipping the last, trailing digit separator.
        None
    } else {
        Some(value)
    }
}

/// Consumes internal and leading digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_il<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This will only consume a single digit separator that is not trailing.
    let value = iter.iter.next()?;
    if not_t(value, iter) {
        iter.next()
    } else {
        // Invalid trailing digit separator or valid value, have to yield it.
        Some(value)
    }
}

/// Consumes internal and trailing digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_it<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This will only consume a single digit separator that is not leading.
    let value = iter.iter.next()?;
    let skip = not_l(value, iter);
    iter.has_started = true;
    if skip {
        // If it's trailing, this will always be None.
        // If it's internal, will yield the next value.
        iter.next()
    } else {
        // Invalid leading digit separator or valid value.
        Some(value)
    }
}

/// Consumes leading and trailing digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_lt<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This will only consume a single digit separator that is not internal.
    let value = iter.iter.next()?;
    let skip = not_i(value, iter);
    iter.has_started = true;
    if skip {
        // If it's trailing, this will always be None.
        // If it's leading, will yield the next value.
        iter.next()
    } else {
        // Invalid internal digit separator or valid value.
        Some(value)
    }
}

/// Consumes internal, leading, and trailing digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_ilt<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This consume any digit separator except consecutive ones.
    let value = iter.iter.next()?;
    if *value == iter.skip {
        iter.next()
    } else {
        Some(value)
    }
}

/// Consumes internal and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_ic<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // Skips as many internal digit separators.
    loop {
        let value = iter.iter.next()?;
        let skip = is_i(value, iter);
        iter.has_started = true;
        if !skip {
            // Not an internal digit separator, leave early.
            return Some(value);
        }
    }
}

/// Consumes leading and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_lc<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // Skips as many leading digit separators as possible.
    loop {
        let value = iter.iter.next()?;
        if !is_l(value, iter) {
            // Not a leading separator, return our value.
            iter.has_started = true;
            return Some(value);
        }
    }
}

/// Consumes trailing and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_tc<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    let value = iter.iter.next()?;
    // Handle the special case of trailing digit separators:
    // All remaining elements in the slice must be empty.
    if value == &iter.skip && iter.iter.as_slice().iter().all(|x| x == &iter.skip) {
        // Is a trailing digit separator, set the iterator to empty, and return None.
        let slc = iter.iter.as_slice();
        // SAFETY: always safety, since the length must be in bounds.
        let slc = unsafe { slc.get_unchecked(slc.len()..) };
        iter.iter = slc.iter();
        None
    } else {
        Some(value)
    }
}

/// Consumes internal, leading, and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_ilc<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    loop {
        let value = iter.iter.next()?;
        // Need to determine if the value is an invalid, trailing digit separator.
        if value != &iter.skip {
            return Some(value);
        } else if iter.iter.as_slice().iter().all(|x| x == &iter.skip) {
            // Invalid trailing digit separator.
            return Some(value);
        }
    }
}

/// Consumes internal, trailing, and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_itc<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // Basically, can skip anything that's not a leading digit separator.
    loop {
        let value = iter.iter.next()?;
        let has_started = iter.has_started;
        iter.has_started = true;
        // Need to determine if the value is an invalid, leading digit separator.
        if value != &iter.skip {
            return Some(value);
        } else if !has_started {
            // Invalid leading digit separator.
            return Some(value);
        }
    }
}

/// Consumes leading, trailing, and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_ltc<'a, T, const FORMAT: u128>(iter: &mut SkipValueIterator<'a, T, FORMAT>) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // Handle leading and trailing differently: only cannot skip
    // internal digit separators. Here, we skip all leading
    // digit separators.
    while !iter.has_started && iter.iter.peek() == Some(&iter.skip) {
        iter.next();
    }

    // Now, only need to handle trailing digit separators.
    next_tc(iter)
}

/// Consumes internal, leading, trailing, and consecutive digit separators.
/// Yields the next token that's not a digit separator.
#[inline]
fn next_iltc<'a, T, const FORMAT: u128>(
    iter: &mut SkipValueIterator<'a, T, FORMAT>,
) -> Option<&'a T>
where
    T: 'a + PartialEq + Clone,
{
    // This consumes any and all digit separators.
    loop {
        let value = iter.iter.next()?;
        if *value != iter.skip {
            return Some(value);
        }
    }
}

// SKIP
// ----

/// Slice iterator that skips characters matching a given value.
///
/// This wraps an iterator over a contiguous block of memory,
/// and only returns values that are not equal to skip.
///
/// The format allows us to dictate the actual behavior of
/// the iterator: in what contexts does it skip digit separators.
#[derive(Clone)]
pub struct SkipValueIterator<'a, T, const FORMAT: u128>
where
    T: 'a + PartialEq + Clone,
{
    /// Slice iterator to wrap.
    iter: slice::Iter<'a, T>,
    /// Value to skip.
    skip: T,
    /// If the iterator has started internally.
    /// This value might not be set for all skip digit implementations:
    /// only ones that skip leading or internal, but not other digit
    /// separators need this for bookkeeping.
    has_started: bool,
    // TODO(ahuszagh) Might need a more complex data structure here...
}

impl<'a, T, const FORMAT: u128> SkipValueIterator<'a, T, FORMAT>
where
    T: 'a + PartialEq + Clone,
{
    #[inline]
    pub fn new(slc: &'a [T], skip: T) -> Self {
        SkipValueIterator {
            iter: slc.iter(),
            skip,
            has_started: false,
        }
    }
}

impl<'a, T, const FORMAT: u128> iter::Iterator for SkipValueIterator<'a, T, FORMAT>
where
    T: 'a + PartialEq + Clone,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO(ahuszagh) This a dummy implementation, will do later correctly...
        match FORMAT {
            0x1 => next_i(self),
            0x2 => next_l(self),
            0x3 => next_t(self),
            0x4 => next_il(self),
            0x5 => next_it(self),
            0x6 => next_lt(self),
            0x7 => next_ilt(self),
            0x8 => next_ic(self),
            0x9 => next_lc(self),
            0xA => next_tc(self),
            0xB => next_ilc(self),
            0xC => next_itc(self),
            0xD => next_ltc(self),
            0xE => next_iltc(self),
            _ => unreachable!(),
        }
    }
}

impl<'a, T, const FORMAT: u128> Iterator<'a, T> for SkipValueIterator<'a, T, FORMAT>
where
    T: 'a + PartialEq + Clone,
{
    const IS_CONTIGUOUS: bool = false;

    #[inline]
    fn new(slc: &'a [T], skip: T) -> Self {
        SkipValueIterator::new(slc, skip)
    }

    #[inline]
    fn from_slice(&self, slc: &'a [T]) -> Self {
        SkipValueIterator::new(slc, self.skip.clone())
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [T] {
        self.iter.as_slice()
    }

    #[inline]
    fn is_consumed(&mut self) -> bool {
        todo!();
    }

    #[inline]
    fn is_empty(&self) -> bool {
        todo!();
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> Self::Item {
        todo!();
    }

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        // TODO(ahuszagh) This needs to handle the complicated next rules...
        //        // Advance the iterator state to the next value,
        //        // but don't consume it.
        //        loop {
        //            let value = self.iter.peek()?;
        //            if *value == self.skip {
        //                self.iter.next();
        //            } else {
        //                return Some(value);
        //            }
        //        }
        todo!();
    }
}

// TODO(ahuszagh) Now need to implement the actual iterator format for it...
