//! An iterator that skips values equal to a provided value.
//!
//! SkipIterator iterates over a slice, returning all values
//! except for those matching the provided skip value.
//!
//! # Complexity
//!
//! Although superficially quite simple, the level of complexity
//! introduced by digit separators can be quite complex, due
//! the number of permutations during parsing.
//!
//! We can consume any combinations of of \[0,3\] items from the following set:
//!     - \[l\]eading digit separators, where digit separators occur before digits.
//!     - \[i\]nternal digit separators, where digit separators occur between digits.
//!     - \[t\]railing digit separators, where digit separators occur after digits.
//!
//! In addition to those combinations, we can also have:
//!     - \[c\]onsecutive digit separators, which allows two digit separators to be adjacent.
//!
//! # Shorthand
//!
//! We will use the term consumer to denote a function that consumes digits,
//! splitting an input buffer at an index, where the leading section contains
//! valid input digits, and the trailing section contains invalid characters.
//! Due to the number of combinations for consumers, we use the following
//! shorthand to denote consumers:
//!     - `no`, does not use a digit separator.
//!     - `l`, consumes leading digit separators.
//!     - `i`, consumes internal digit separators.
//!     - `t`, consumes trailing digit separators.
//!     - `c`, consumes consecutive digit separators.
//!
//! The `next`/`iter` algorithms are therefore named `next_x`, where `x`
//! represents the shorthand name of the consumer, in sorted order.
//!  For example, `next_ilt` means that consumer can skip internal,
//! leading, and trailing digit separators, but not consecutive ones.

#![cfg(all(feature = "format", feature = "parse"))]

use crate::digit::char_is_digit_const;
use crate::iterator::ByteIter;

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

// SKIP ITER
// ---------

/// Trait to simplify creation of a `SkipIterator`.
pub trait SkipIter<'a> {
    /// Create `SkipIterator` from format and current type.
    fn skip_iter<const RADIX: u32, const FORMAT: u128>(&'a self)
        -> SkipIterator<'a, RADIX, FORMAT>;
}

impl<'a> SkipIter<'a> for [u8] {
    #[inline]
    fn skip_iter<const RADIX: u32, const FORMAT: u128>(
        &'a self,
    ) -> SkipIterator<'a, RADIX, FORMAT> {
        SkipIterator::new(self)
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
///
/// `RADIX` is required to allow us to differentiate digit from
/// non-digit characters (see [DigitSeparators](/docs/DigitSeparators.md)
/// for a detailed explanation on why), and `FORMAT` is required to tell
/// us what the digit separator is, and where the digit separator
#[derive(Clone)]
pub struct SkipIterator<'a, const RADIX: u32, const FORMAT: u128> {
    /// The raw slice for the iterator.
    slc: &'a [u8],
    /// Current index of the iterator in the slice.
    index: usize,
}

impl<'a, const RADIX: u32, const FORMAT: u128> SkipIterator<'a, RADIX, FORMAT> {
    /// Create new iterator.
    #[inline]
    pub fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }

    /// Determine if the character is a digit separator.
    pub const fn is_digit_separator(&self, value: u8) -> bool {
        if digit_separator::<FORMAT>() == 0 {
            // Check at compile time if we have an invalid digit separator.
            // b'\x00', or the NUL character, is this invalid value.
            false
        } else {
            value == digit_separator::<FORMAT>()
        }
    }

    /// Determine if the character is a digit.
    pub const fn is_digit(&self, value: u8) -> bool {
        char_is_digit_const(value, RADIX)
    }
}

impl<'a, const RADIX: u32, const FORMAT: u128> Iterator for SkipIterator<'a, RADIX, FORMAT> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Peek will handle everything properly internally.
        let value = self.peek()?;
        // Increment the index so we know not to re-fetch it.
        self.index += 1;
        Some(value)
    }
}

impl<'a, const RADIX: u32, const FORMAT: u128> ByteIter<'a> for SkipIterator<'a, RADIX, FORMAT> {
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
        // SAFETY: safe since index must be in range
        unsafe { self.slc.get_unchecked(self.index..) }
    }

    #[inline]
    fn length(&self) -> usize {
        self.slc.len()
    }

    #[inline]
    fn cursor(&self) -> usize {
        self.index
    }

    #[inline]
    fn is_consumed(&mut self) -> bool {
        self.peek().is_none()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.index >= self.slc.len()
    }

    // NOTE: panics if the peeked value isn't valid.
    #[inline]
    unsafe fn peek_unchecked(&mut self) -> Self::Item {
        self.peek().unwrap()
    }

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        // TODO(ahuszagh) This a dummy implementation, will do later correctly...
        match FORMAT {
            I => peek_i(self),
            L => peek_l(self),
            T => peek_t(self),
            IL => peek_il(self),
            IT => peek_it(self),
            LT => peek_lt(self),
            ILT => peek_ilt(self),
            IC => peek_ic(self),
            LC => peek_lc(self),
            TC => peek_tc(self),
            ILC => peek_ilc(self),
            ITC => peek_itc(self),
            LTC => peek_ltc(self),
            ILTC => peek_iltc(self),
            _ => unreachable!(),
        }
    }

    #[inline]
    unsafe fn step_by_unchecked(&mut self, _: usize) {
        unimplemented!("Not a contiguous iterator.");
    }
}

// PEEK
// ----

/// Determine if the digit separator is internal.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
/// The compiler optimizes this pretty well: it's almost as efficient as
/// optimized assembly without bounds checking.
#[inline]
fn is_i<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> bool {
    !is_l(iter) && !is_t(iter)
}

/// Determine if the digit separator is leading.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
/// The compiler optimizes this pretty well: it's almost as efficient as
/// optimized assembly without bounds checking.
#[inline]
fn is_l<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> bool {
    // Consume any digit separators before the current one.
    let mut index = iter.index;
    while index > 0 && iter.slc.get(index - 1).map_or(false, |&x| iter.is_digit_separator(x)) {
        index -= 1;
    }

    // True if there are no items before the digit separator, or character
    // before the digit separators is not a digit.
    index == 0 || !iter.slc.get(index - 1).map_or(false, |&x| iter.is_digit(x))
}

/// Determine if the digit separator is trailing.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
/// The compiler optimizes this pretty well: it's almost as efficient as
/// optimized assembly without bounds checking.
#[inline]
fn is_t<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> bool {
    // Consume any digit separators after the current one.
    let mut index = iter.index;
    while index < iter.slc.len()
        && iter.slc.get(index + 1).map_or(false, |&x| iter.is_digit_separator(x))
    {
        index += 1;
    }

    index == iter.slc.len() || !iter.slc.get(index + 1).map_or(false, |&x| iter.is_digit(x))
}

/// Determine if the digit separator is leading or internal.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
#[inline]
fn is_il<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> bool {
    is_l(iter) || !is_t(iter)
}

/// Determine if the digit separator is internal or trailing.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
#[inline]
fn is_it<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> bool {
    is_t(iter) || !is_l(iter)
}

/// Determine if the digit separator is leading or trailing.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
#[inline]
fn is_lt<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> bool {
    is_l(iter) || is_t(iter)
}

/// Consumes at most 1 digit separator.
/// Peeks the next token that's not a digit separator.
#[inline]
fn peek_1<'a, Callback, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
    is_skip: Callback,
) -> Option<&'a u8>
where
    Callback: Fn(&mut SkipIterator<'a, RADIX, FORMAT>) -> bool,
{
    // This will only consume a single digit separator.
    // This will not consume consecutive digit separators.
    let value = iter.slc.get(iter.index)?;
    let is_digit_separator = iter.is_digit_separator(*value);
    if is_digit_separator && is_skip(iter) {
        // Have a skippable digit separator: increment the index and skip to
        // the next value, which we cannot skip.
        iter.index += 1;
        iter.slc.get(iter.index)
    } else {
        // Have 1 of 2 conditions:
        //  1. A non-digit separator character.
        //  2. A digit separator that is not valid in the context.
        Some(value)
    }
}

/// Consumes 1 or more digit separators.
/// Peeks the next token that's not a digit separator.
#[inline]
fn peek_n<'a, Callback, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
    is_skip: Callback,
) -> Option<&'a u8>
where
    Callback: Fn(&mut SkipIterator<'a, RADIX, FORMAT>) -> bool,
{
    // This will consume consecutive digit separators.
    let value = iter.slc.get(iter.index)?;
    let is_digit_separator = iter.is_digit_separator(*value);
    if is_digit_separator && is_skip(iter) {
        // Have a skippable digit separator: keep incrementing until we find
        // a non-digit separator character. Don't need any complex checks
        // here, since we've already done them above.
        let mut index = iter.index + 1;
        while index < iter.slc.len()
            && iter.slc.get(index).map_or(false, |&x| iter.is_digit_separator(x))
        {
            index += 1;
        }
        iter.index = index;
        iter.slc.get(iter.index)
    } else {
        // Have 1 of 2 conditions:
        //  1. A non-digit separator character.
        //  2. A digit separator that is not valid in the context.
        Some(value)
    }
}

/// Consumes at most 1 leading digit separator and peeks the next value.
#[inline]
fn peek_l<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, is_l)
}

/// Consumes at most 1 internal digit separator and peeks the next value.
#[inline]
fn peek_i<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, is_i)
}

/// Consumes at most 1 trailing digit separator and peeks the next value.
#[inline]
fn peek_t<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, is_t)
}

/// Consumes at most 1 internal/leading digit separator and peeks the next value.
#[inline]
fn peek_il<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, is_il)
}

/// Consumes at most 1 internal/trailing digit separator and peeks the next value.
#[inline]
fn peek_it<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, is_it)
}

/// Consumes at most 1 leading/trailing digit separator and peeks the next value.
#[inline]
fn peek_lt<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, is_lt)
}

/// Consumes at most 1 digit separator and peeks the next value.
#[inline]
fn peek_ilt<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_1(iter, |_| true)
}

/// Consumes 1 or more leading digit separators and peeks the next value.
#[inline]
fn peek_lc<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, is_l)
}

/// Consumes 1 or more internal digit separators and peeks the next value.
#[inline]
fn peek_ic<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, is_i)
}

/// Consumes 1 or more trailing digit separators and peeks the next value.
#[inline]
fn peek_tc<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, is_t)
}

/// Consumes 1 or more internal/leading digit separators and peeks the next value.
#[inline]
fn peek_ilc<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, is_il)
}

/// Consumes 1 or more internal/trailing digit separators and peeks the next value.
#[inline]
fn peek_itc<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, is_it)
}

/// Consumes 1 or more leading/trailing digit separators and peeks the next value.
#[inline]
fn peek_ltc<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, is_lt)
}

/// Consumes 1 or more digit separators and peeks the next value.
#[inline]
fn peek_iltc<'a, const RADIX: u32, const FORMAT: u128>(
    iter: &mut SkipIterator<'a, RADIX, FORMAT>,
) -> Option<&'a u8> {
    peek_n(iter, |_| true)
}
