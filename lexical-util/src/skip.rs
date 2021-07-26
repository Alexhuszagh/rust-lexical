//! An iterator that skips values equal to a provided value.
//!
//! SkipIterator iterates over a slice, returning all values
//! except for those matching the provided skip value.
//!
//! Example
//! -------
//!
//! ```text
//! let iter = SkipIterator(&[1, 2, 5, 2, 6, 7], 2);
//! assert!(iter.eq([1, 5, 6, 7].iter()));
//! ```

#![cfg(all(feature = "format", feature = "parse"))]

use crate::iterator::ByteIter;
use crate::lib::slice;

// TODO(ahuszagh) Remove these:
//  Temporary constants to enable our logic to work.
pub const I: u128 = 0x1;
pub const L: u128 = 0x2;
pub const T: u128 = 0x3;
pub const IL: u128 = 0x4;
pub const IT: u128 = 0x5;
pub const LT: u128 = 0x6;
pub const ILT: u128 = 0x7;
pub const IC: u128 = 0x8;
pub const LC: u128 = 0x9;
pub const TC: u128 = 0xA;
pub const ILC: u128 = 0xB;
pub const ITC: u128 = 0xC;
pub const LTC: u128 = 0xD;
pub const ILTC: u128 = 0xE;

// TODO(ahuszagh) Actually implement...
const fn digit_separator<const FORMAT: u128>() -> u8 {
    b'_'
}

const fn decimal_point<const FORMAT: u128>() -> u8 {
    b'.'
}

const fn exponent_character<const FORMAT: u128>() -> u8 {
    b'e'
}

// SKIP ITER
// ---------

/// Trait to simplify creation of a `SkipIterator`.
pub trait SkipIter<'a>: IntoIterator<Item = &'a u8>
{
    /// Create `SkipIterator` from format and current type.
    fn skip_iter<const FORMAT: u128>(&'a self) -> SkipIterator<'a, FORMAT>;
}

impl<'a> SkipIter<'a> for &'a [u8] {
    #[inline]
    fn skip_iter<const FORMAT: u128>(&'a self) -> SkipIterator<'a, FORMAT> {
        SkipIterator::new(self)
    }
}

// NEXT/PEEK
// ---------

// TODO(ahuszagh) Restore...
/// Determine if the value is an internal digit separator.
// TODO(ahuszagh) Make it a method FFS...
#[inline]
fn is_i<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> bool
{
    // Need to check the value is not internal or trailing skip digit.
    iter.has_started && iter.iter.slice_len() != 0
}

// TODO(ahuszagh) Restore...
///// Determine if the value is a leading digit separator.
//#[inline]
//fn is_l<'a, const FORMAT: u128>(value: u8, iter: &mut SkipIterator<'a, FORMAT>) -> bool
//{
//    !iter.has_started && value == digit_separator::<FORMAT>()
//}
//
///// Determine if the value is a trailing digit separator.
//#[inline]
//fn is_t<'a, const FORMAT: u128>(value: u8, iter: &mut SkipIterator<'a, FORMAT>) -> bool
//{
//    iter.iter.slice_len() == 0 && value == digit_separator::<FORMAT>()
//}
//
///// Determine if the value is not an internal digit separator.
//#[inline]
//fn not_i<'a, const FORMAT: u128>(value: u8, iter: &mut SkipIterator<'a, FORMAT>) -> bool
//{
//    if value == digit_separator::<FORMAT>() {
//        !iter.has_started || iter.iter.slice_len() == 0
//    } else {
//        true
//    }
//}
//
///// Determine if the value is not an leading digit separator.
//#[inline]
//fn not_l<'a, const FORMAT: u128>(value: u8, iter: &mut SkipIterator<'a, FORMAT>) -> bool
//{
//    iter.has_started || value != digit_separator::<FORMAT>()
//}
//
///// Determine if the value is not a trailing digit separator.
//#[inline]
//fn not_t<'a, const FORMAT: u128>(value: u8, iter: &mut SkipIterator<'a, FORMAT>) -> bool
//{
//    iter.iter.slice_len() != 0 || value != digit_separator::<FORMAT>()
//}
//
//// These consume 1 or more valid digit separators and produce the next
//// value from the input data.
//
///// Consumes internal digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_i<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // Skips a single internal, but not leading or trailing, digit separator.
//    // This means if the consecutive item is a digit separator, then we yield
//    // that value.
//    let value = iter.iter.next()?;
//    let is_skip = *value == digit_separator::<FORMAT>();
//    let should_skip = is_i(is_skip, iter);
//    iter.has_started = !is_skip;
//    if should_skip {
//        iter.next()
//    } else {
//        // Might be a digit separator, might not be: either way, we can't skip it.
//        Some(value)
//    }
//}
//
///// Consumes internal digit separators.
///// Peeks the next token that's not a digit separator.
//#[inline]
//fn peek_i<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // Skips a single internal, but not leading or trailing, digit separator.
//    // This means if the consecutive item is a digit separator, then we yield
//    // that value.
//    let value = iter.iter.peek()?;
//    let is_skip = *value == digit_separator::<FORMAT>();
//    let should_skip = is_i(is_skip, iter);
//    if should_skip {
//        iter.next();
//        iter.has_started = !is_skip;
//        iter.peek()
//    } else {
//        // Might be a digit separator, might not be: either way, we can't skip it.
//        // Can't set `has_started`, since we haven't consumed the value.
//        Some(value)
//    }
//}
//
///// Consumes leading digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_l<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // This will only consume a single digit separator at the start of the slice.
//    let value = iter.iter.next()?;
//    let skip = is_l(value, iter);
//    iter.has_started = true;
//    if skip {
//        iter.next()
//    } else {
//        // Might be a digit separator, might not be: either way, we can't skip it.
//        Some(value)
//    }
//}
//
///// Consumes trailing digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_t<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // This will only consume a single digit separator at the end of the slice.
//    let value = iter.iter.next()?;
//    if is_t(value, iter) {
//        // Skipping the last, trailing digit separator.
//        None
//    } else {
//        Some(value)
//    }
//}
//
///// Consumes internal and leading digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_il<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // This will only consume a single digit separator that is not trailing.
//    let value = iter.iter.next()?;
//    if not_t(value, iter) {
//        iter.next()
//    } else {
//        // Invalid trailing digit separator or valid value, have to yield it.
//        Some(value)
//    }
//}
//
///// Consumes internal and trailing digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_it<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // This will only consume a single digit separator that is not leading.
//    let value = iter.iter.next()?;
//    let skip = not_l(value, iter);
//    iter.has_started = true;
//    if skip {
//        // If it's trailing, this will always be None.
//        // If it's internal, will yield the next value.
//        iter.next()
//    } else {
//        // Invalid leading digit separator or valid value.
//        Some(value)
//    }
//}
//
///// Consumes leading and trailing digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_lt<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // This will only consume a single digit separator that is not internal.
//    let value = iter.iter.next()?;
//    let skip = not_i(value, iter);
//    iter.has_started = true;
//    if skip {
//        // If it's trailing, this will always be None.
//        // If it's leading, will yield the next value.
//        iter.next()
//    } else {
//        // Invalid internal digit separator or valid value.
//        Some(value)
//    }
//}
//
///// Consumes internal, leading, and trailing digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_ilt<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // This consume any digit separator except consecutive ones.
//    let value = iter.iter.next()?;
//    if *value == iter.skip {
//        iter.next()
//    } else {
//        Some(value)
//    }
//}
//
///// Consumes internal and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_ic<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//where
//    T: 'a + PartialEq + Clone,
//{
//    // Skips as many internal digit separators.
//    loop {
//        let value = iter.iter.next()?;
//        let skip = is_i(value, iter);
//        iter.has_started = true;
//        if !skip {
//            // Not an internal digit separator, leave early.
//            return Some(value);
//        }
//    }
//}
//
///// Consumes leading and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_lc<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // Skips as many leading digit separators as possible.
//    loop {
//        let value = iter.iter.next()?;
//        if !is_l(value, iter) {
//            // Not a leading separator, return our value.
//            iter.has_started = true;
//            return Some(value);
//        }
//    }
//}
//
///// Consumes trailing and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_tc<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    let value = iter.iter.next()?;
//    // Handle the special case of trailing digit separators:
//    // All remaining elements in the slice must be empty.
//    if value == &iter.skip && iter.iter.as_slice().iter().all(|x| x == &iter.skip) {
//        // Is a trailing digit separator, set the iterator to empty, and return None.
//        let slc = iter.iter.as_slice();
//        // SAFETY: always safety, since the length must be in bounds.
//        let slc = unsafe { slc.get_unchecked(slc.len()..) };
//        iter.iter = slc.iter();
//        None
//    } else {
//        Some(value)
//    }
//}
//
///// Consumes internal, leading, and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_ilc<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//where
//    T: 'a + PartialEq + Clone,
//{
//    loop {
//        let value = iter.iter.next()?;
//        // Need to determine if the value is an invalid, trailing digit separator.
//        if value != &iter.skip {
//            return Some(value);
//        } else if iter.iter.as_slice().iter().all(|x| x == &iter.skip) {
//            // Invalid trailing digit separator.
//            return Some(value);
//        }
//    }
//}
//
///// Consumes internal, trailing, and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_itc<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//where
//    T: 'a + PartialEq + Clone,
//{
//    // Basically, can skip anything that's not a leading digit separator.
//    loop {
//        let value = iter.iter.next()?;
//        let has_started = iter.has_started;
//        iter.has_started = true;
//        // Need to determine if the value is an invalid, leading digit separator.
//        if value != &iter.skip {
//            return Some(value);
//        } else if !has_started {
//            // Invalid leading digit separator.
//            return Some(value);
//        }
//    }
//}
//
///// Consumes leading, trailing, and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_ltc<'a, const FORMAT: u128>(iter: &mut SkipIterator<'a, FORMAT>) -> Option<&'a u8>
//{
//    // Handle leading and trailing differently: only cannot skip
//    // internal digit separators. Here, we skip all leading
//    // digit separators.
//    while !iter.has_started && iter.iter.peek() == Some(&iter.skip) {
//        iter.next();
//    }
//
//    // Now, only need to handle trailing digit separators.
//    next_tc(iter)
//}
//
///// Consumes internal, leading, trailing, and consecutive digit separators.
///// Yields the next token that's not a digit separator.
//#[inline]
//fn next_iltc<'a, const FORMAT: u128>(
//    iter: &mut SkipIterator<'a, FORMAT>,
//) -> Option<&'a u8>
//{
//    // This consumes any and all digit separators.
//    loop {
//        let value = iter.iter.next()?;
//        if *value != iter.skip {
//            return Some(value);
//        }
//    }
//}

// SKIP
// ----

// TODO(ahuszagh) Later on can probably remove the `skip` part, since
// it's just a const generic part of the format.

/// Slice iterator that skips characters matching a given value.
///
/// This wraps an iterator over a contiguous block of memory,
/// and only returns values that are not equal to skip.
///
/// The format allows us to dictate the actual behavior of
/// the iterator: in what contexts does it skip digit separators.
#[derive(Clone)]
pub struct SkipIterator<'a, const FORMAT: u128>
{
    /// Slice iterator to wrap.
    iter: slice::Iter<'a, u8>,
    /// If the iterator has started internally.
    /// This value might not be set for all skip digit implementations:
    /// only ones that skip leading or internal, but not other digit
    /// separators need this for bookkeeping.
    has_started: bool,
    // TODO(ahuszagh) Might need a more complex data structure here...
}

impl<'a, const FORMAT: u128> SkipIterator<'a, FORMAT>
{
    /// Create new iterator.
    #[inline]
    pub fn new(slc: &'a [u8]) -> Self {
        SkipIterator {
            iter: slc.iter(),
            has_started: false,
        }
    }

    /// Get the value to skip from the format.
    pub const fn skip() -> u8 {
        digit_separator::<FORMAT>()
    }
}

impl<'a, const FORMAT: u128> Iterator for SkipIterator<'a, FORMAT>
where
{
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO(ahuszagh) This a dummy implementation, will do later correctly...
        match FORMAT {
            //I => next_i(self),
            //L => next_l(self),
            //T => next_t(self),
            //IL => next_il(self),
            //IT => next_it(self),
            //LT => next_lt(self),
            //ILT => next_ilt(self),
            //IC => next_ic(self),
            //LC => next_lc(self),
            //TC => next_tc(self),
            //ILC => next_ilc(self),
            //ITC => next_itc(self),
            //LTC => next_ltc(self),
            //ILTC => next_iltc(self),
            _ => unreachable!(),
        }
    }
}

impl<'a, const FORMAT: u128> ByteIter<'a> for SkipIterator<'a, FORMAT>
{
    const IS_CONTIGUOUS: bool = false;

    #[inline]
    fn new(slc: &'a [u8]) -> Self {
        SkipIterator::new(slc)
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [u8] {
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
        // TODO(ahuszagh) This a dummy implementation, will do later correctly...
        match FORMAT {
            //I => peek_i(self),
            L => todo!(),
            T => todo!(),
            IL => todo!(),
            IT => todo!(),
            LT => todo!(),
            ILT => todo!(),
            IC => todo!(),
            LC => todo!(),
            TC => todo!(),
            ILC => todo!(),
            ITC => todo!(),
            LTC => todo!(),
            ILTC => todo!(),
            _ => unreachable!(),
        }
    }
}


// TODO(ahuszagh) Now need to implement the actual iterator format for it...
