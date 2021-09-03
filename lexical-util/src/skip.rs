//! An iterator that skips values equal to a provided value.
//!
//! Iterators over a contiguous slice, returning all values
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
use crate::format::NumberFormat;
use crate::format_flags as flags;
use crate::iterator::BytesIter;
use core::{mem, ptr};

// PEEK
// ----

/// Determine if the digit separator is internal.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
/// The compiler optimizes this pretty well: it's almost as efficient as
/// optimized assembly without bounds checking.
macro_rules! is_i {
    ($self:ident) => {
        !is_l!($self) && !is_t!($self)
    };
}

/// Determine if the digit separator is leading.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
/// The compiler optimizes this pretty well: it's almost as efficient as
/// optimized assembly without bounds checking.
macro_rules! is_l {
    ($self:ident) => {{
        // Consume any digit separators before the current one.
        let mut index = $self.byte.index;
        while index > 0
            && $self.byte.slc.get(index - 1).map_or(false, |&x| $self.is_digit_separator(x))
        {
            index -= 1;
        }

        // True if there are no items before the digit separator, or character
        // before the digit separators is not a digit.
        index == 0 || !$self.byte.slc.get(index - 1).map_or(false, |&x| $self.is_digit(x))
    }};
}

/// Determine if the digit separator is trailing.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
/// The compiler optimizes this pretty well: it's almost as efficient as
/// optimized assembly without bounds checking.
macro_rules! is_t {
    ($self:ident) => {{
        // Consume any digit separators after the current one.
        let mut index = $self.byte.index;
        while index < $self.byte.slc.len()
            && $self.byte.slc.get(index + 1).map_or(false, |&x| $self.is_digit_separator(x))
        {
            index += 1;
        }

        index == $self.byte.slc.len()
            || !$self.byte.slc.get(index + 1).map_or(false, |&x| $self.is_digit(x))
    }};
}

/// Determine if the digit separator is leading or internal.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
macro_rules! is_il {
    ($self:ident) => {
        is_l!($self) || !is_t!($self)
    };
}

/// Determine if the digit separator is internal or trailing.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
macro_rules! is_it {
    ($self:ident) => {
        is_t!($self) || !is_l!($self)
    };
}

/// Determine if the digit separator is leading or trailing.
///
/// Preconditions: Assumes `slc[index]` is a digit separator.
macro_rules! is_lt {
    ($self:ident) => {
        is_l!($self) || is_t!($self)
    };
}

/// Determine if the digit separator is internal, leading, or trailing.
macro_rules! is_ilt {
    ($self:ident) => {
        true
    };
}

/// Consumes 1 or more digit separators.
/// Peeks the next token that's not a digit separator.
macro_rules! peek_1 {
    ($self:ident, $is_skip:ident) => {{
        // This will consume consecutive digit separators.
        let value = $self.byte.slc.get($self.byte.index)?;
        let is_digit_separator = $self.is_digit_separator(*value);
        if is_digit_separator && $is_skip!($self) {
            // Have a skippable digit separator: keep incrementing until we find
            // a non-digit separator character. Don't need any complex checks
            // here, since we've already done them above.
            let mut index = $self.byte.index + 1;
            while index < $self.length()
                && $self.byte.slc.get(index).map_or(false, |&x| $self.is_digit_separator(x))
            {
                index += 1;
            }
            $self.byte.index = index;
            $self.byte.slc.get($self.byte.index)
        } else {
            // Have 1 of 2 conditions:
            //  1. A non-digit separator character.
            //  2. A digit separator that is not valid in the context.
            Some(value)
        }
    }};
}

/// Consumes 1 or more digit separators.
/// Peeks the next token that's not a digit separator.
macro_rules! peek_n {
    ($self:ident, $is_skip:ident) => {{
        // This will consume consecutive digit separators.
        let value = $self.byte.slc.get($self.byte.index)?;
        let is_digit_separator = $self.is_digit_separator(*value);
        if is_digit_separator && $is_skip!($self) {
            // Have a skippable digit separator: keep incrementing until we find
            // a non-digit separator character. Don't need any complex checks
            // here, since we've already done them above.
            let mut index = $self.byte.index + 1;
            while index < $self.byte.slc.len()
                && $self.byte.slc.get(index).map_or(false, |&x| $self.is_digit_separator(x))
            {
                index += 1;
            }
            $self.byte.index = index;
            $self.byte.slc.get($self.byte.index)
        } else {
            // Have 1 of 2 conditions:
            //  1. A non-digit separator character.
            //  2. A digit separator that is not valid in the context.
            Some(value)
        }
    }};
}

/// Consumes no digit separators and peeks the next value.
macro_rules! peek_noskip {
    ($self:ident) => {
        $self.byte.slc.get($self.byte.index)
    };
}

/// Consumes at most 1 leading digit separator and peeks the next value.
macro_rules! peek_l {
    ($self:ident) => {
        peek_1!($self, is_l)
    };
}

/// Consumes at most 1 internal digit separator and peeks the next value.
macro_rules! peek_i {
    ($self:ident) => {
        peek_1!($self, is_i)
    };
}

/// Consumes at most 1 trailing digit separator and peeks the next value.
macro_rules! peek_t {
    ($self:ident) => {
        peek_1!($self, is_t)
    };
}

/// Consumes at most 1 internal/leading digit separator and peeks the next value.
macro_rules! peek_il {
    ($self:ident) => {
        peek_1!($self, is_il)
    };
}

/// Consumes at most 1 internal/trailing digit separator and peeks the next value.
macro_rules! peek_it {
    ($self:ident) => {
        peek_1!($self, is_it)
    };
}

/// Consumes at most 1 leading/trailing digit separator and peeks the next value.
macro_rules! peek_lt {
    ($self:ident) => {
        peek_1!($self, is_lt)
    };
}

/// Consumes at most 1 digit separator and peeks the next value.
macro_rules! peek_ilt {
    ($self:ident) => {
        peek_1!($self, is_ilt)
    };
}

/// Consumes 1 or more leading digit separators and peeks the next value.
macro_rules! peek_lc {
    ($self:ident) => {
        peek_n!($self, is_l)
    };
}

/// Consumes 1 or more internal digit separators and peeks the next value.
macro_rules! peek_ic {
    ($self:ident) => {
        peek_n!($self, is_i)
    };
}

/// Consumes 1 or more trailing digit separators and peeks the next value.
macro_rules! peek_tc {
    ($self:ident) => {
        peek_n!($self, is_t)
    };
}

/// Consumes 1 or more internal/leading digit separators and peeks the next value.
macro_rules! peek_ilc {
    ($self:ident) => {
        peek_n!($self, is_il)
    };
}

/// Consumes 1 or more internal/trailing digit separators and peeks the next value.
macro_rules! peek_itc {
    ($self:ident) => {
        peek_n!($self, is_it)
    };
}

/// Consumes 1 or more leading/trailing digit separators and peeks the next value.
macro_rules! peek_ltc {
    ($self:ident) => {
        peek_n!($self, is_lt)
    };
}

/// Consumes 1 or more digit separators and peeks the next value.
macro_rules! peek_iltc {
    ($self:ident) => {{
        loop {
            let value = $self.byte.slc.get($self.byte.index)?;
            if !$self.is_digit_separator(*value) {
                return Some(value);
            }
            $self.byte.index += 1;
        }
    }};
}

// AS DIGITS
// ---------

/// Trait to simplify creation of a `Bytes` object.
pub trait AsBytes<'a> {
    /// Create `Bytes` from object.
    fn bytes<const FORMAT: u128>(&'a self) -> Bytes<'a, FORMAT>;
}

impl<'a> AsBytes<'a> for [u8] {
    #[inline]
    fn bytes<const FORMAT: u128>(&'a self) -> Bytes<'a, FORMAT> {
        Bytes::new(self)
    }
}

// DIGITS
// ------

/// Slice iterator that skips characters matching a given value.
///
/// This wraps an iterator over a contiguous block of memory,
/// and only returns values that are not equal to skip.
///
/// The format allows us to dictate the actual behavior of
/// the iterator: in what contexts does it skip digit separators.
///
/// `FORMAT` is required to tell us what the digit separator is, and where
/// the digit separators are allowed, as well tell us the radix.
/// The radix is required to allow us to differentiate digit from
/// non-digit characters (see [DigitSeparators](/docs/DigitSeparators.md)
/// for a detailed explanation on why).
#[derive(Clone)]
pub struct Bytes<'a, const FORMAT: u128> {
    /// The raw slice for the iterator.
    slc: &'a [u8],
    /// Current index of the iterator in the slice.
    index: usize,
    /// The current count of values returned by the iterator.
    /// This is only used if the iterator is not contiguous.
    count: usize,
}

impl<'a, const FORMAT: u128> Bytes<'a, FORMAT> {
    /// If each yielded value is adjacent in memory.
    pub const IS_CONTIGUOUS: bool = NumberFormat::<{ FORMAT }>::DIGIT_SEPARATOR == 0;

    /// Create new byte object.
    #[inline]
    pub fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
            count: 0,
        }
    }

    /// Get a ptr to the current start of the iterator.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    /// Get a slice to the current start of the iterator.
    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        // SAFETY: safe since index must be in range
        unsafe { self.slc.get_unchecked(self.index..) }
    }

    /// Get the total number of elements in the underlying slice.
    #[inline]
    pub fn length(&self) -> usize {
        self.slc.len()
    }

    /// Get the current index of the iterator in the slice.
    #[inline]
    pub fn cursor(&self) -> usize {
        self.index
    }

    /// Set the current index of the iterator in the slice.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`.
    #[inline]
    pub unsafe fn set_cursor(&mut self, index: usize) {
        debug_assert!(index <= self.length());
        self.index = index
    }

    /// Get the current number of values returned by the iterator.
    #[inline]
    pub fn current_count(&self) -> usize {
        // If the buffer is contiguous, then we don't need to track the
        // number of values: the current index is enough.
        if Self::IS_CONTIGUOUS {
            self.index
        } else {
            self.count
        }
    }

    /// Get if the buffer underlying the iterator is empty.
    ///
    /// This might not be the same thing as `is_consumed`: `is_consumed`
    /// checks if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of `is_consumed()`.
    #[inline]
    pub fn is_done(&self) -> bool {
        self.index >= self.slc.len()
    }

    // Determine if the abstraction is contiguous.
    #[inline]
    pub fn is_contiguous(&self) -> bool {
        Self::IS_CONTIGUOUS
    }

    /// Read a value of a difference type from the iterator.
    /// This advances the internal state of the iterator.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V.
    #[inline]
    pub unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());

        let slc = self.as_slice();
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { ptr::read_unaligned::<V>(slc.as_ptr() as *const _) }
    }

    /// Try to read a value of a different type from the iterator.
    /// This advances the internal state of the iterator.
    #[inline]
    pub fn read<V>(&self) -> Option<V> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<V>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read.
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }

    /// Check if the next element is a given value.
    #[inline]
    pub fn first_is(&mut self, value: u8) -> bool {
        // Don't assert not a digit separator, since this can occur when
        // a different component does not allow digit separators there.
        if let Some(&c) = self.slc.get(self.index) {
            c == value
        } else {
            false
        }
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline]
    pub fn case_insensitive_first_is(&mut self, value: u8) -> bool {
        // Don't assert not a digit separator, since this can occur when
        // a different component does not allow digit separators there.
        if let Some(&c) = self.slc.get(self.index) {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    /// Get iterator over integer digits.
    #[inline]
    pub fn integer_iter<'b>(&'b mut self) -> IntegerBytesIterator<'a, 'b, FORMAT> {
        IntegerBytesIterator {
            byte: self,
        }
    }

    /// Get iterator over fraction digits.
    #[inline]
    pub fn fraction_iter<'b>(&'b mut self) -> FractionBytesIterator<'a, 'b, FORMAT> {
        FractionBytesIterator {
            byte: self,
        }
    }

    /// Get iterator over exponent digits.
    #[inline]
    pub fn exponent_iter<'b>(&'b mut self) -> ExponentBytesIterator<'a, 'b, FORMAT> {
        ExponentBytesIterator {
            byte: self,
        }
    }

    /// Get iterator over special floating point values.
    #[inline]
    pub fn special_iter<'b>(&'b mut self) -> SpecialBytesIterator<'a, 'b, FORMAT> {
        SpecialBytesIterator {
            byte: self,
        }
    }

    /// Advance the byte by `N` elements.
    ///
    /// # Safety
    ///
    /// As long as the iterator is at least `N` elements, this
    /// is safe.
    #[inline]
    pub unsafe fn step_by_unchecked(&mut self, count: usize) {
        if Self::IS_CONTIGUOUS {
            // Contiguous, can skip most of these checks.
            debug_assert!(self.as_slice().len() >= count);
        } else {
            // Since this isn't contiguous, it only works
            // if the value is in the range `[0, 1]`.
            // We also need to make sure the **current** value
            // isn't a digit separator.
            let format = NumberFormat::<{ FORMAT }> {};
            debug_assert!(self.as_slice().len() >= count);
            debug_assert!(count == 0 || count == 1);
            debug_assert!(
                count == 0 || self.slc.get(self.index) != Some(&format.digit_separator())
            );
        }
        self.index += count;
        if !Self::IS_CONTIGUOUS {
            // Only increment the count if it's not contiguous, otherwise,
            // this is an unnecessary performance penalty.
            self.count += count;
        }
    }

    /// Advance the byte by 1 element.
    ///
    /// # Safety
    ///
    /// Safe as long as the iterator is not empty.
    #[inline]
    pub unsafe fn step_unchecked(&mut self) {
        debug_assert!(!self.as_slice().is_empty());
        // SAFETY: safe if `self.index < self.length()`.
        unsafe { self.step_by_unchecked(1) };
    }
}

// ITERATOR HELPERS
// ----------------

/// Create skip iterator definition.
macro_rules! skip_iterator {
    ($iterator:ident, $doc:literal) => {
        #[doc = $doc]
        pub struct $iterator<'a: 'b, 'b, const FORMAT: u128> {
            /// The internal byte object for the skip iterator.
            byte: &'b mut Bytes<'a, FORMAT>,
        }
    };
}

macro_rules! is_digit_separator {
    ($format:ident) => {
        /// Determine if the character is a digit separator.
        pub const fn is_digit_separator(&self, value: u8) -> bool {
            let format = NumberFormat::<{ $format }> {};
            let digit_separator = format.digit_separator();
            if digit_separator == 0 {
                // Check at compile time if we have an invalid digit separator.
                // b'\x00', or the NUL character, is this invalid value.
                false
            } else {
                value == digit_separator
            }
        }
    };
}

/// Create impl block for skip iterator.
macro_rules! skip_iterator_impl {
    ($iterator:ident, $radix_cb:ident) => {
        impl<'a: 'b, 'b, const FORMAT: u128> $iterator<'a, 'b, FORMAT> {
            is_digit_separator!(FORMAT);

            /// Determine if the character is a digit.
            pub const fn is_digit(&self, value: u8) -> bool {
                let format = NumberFormat::<{ FORMAT }> {};
                char_is_digit_const(value, format.$radix_cb())
            }
        }
    };
}

/// Create impl Iterator block for skip iterator.
macro_rules! skip_iterator_iterator_impl {
    ($iterator:ident) => {
        impl<'a: 'b, 'b, const FORMAT: u128> Iterator for $iterator<'a, 'b, FORMAT> {
            type Item = &'a u8;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                // Peek will handle everything properly internally.
                let value = self.peek()?;
                // Increment the index so we know not to re-fetch it.
                self.byte.index += 1;
                if !Self::IS_CONTIGUOUS {
                    // Only increment the count if it's not contiguous, otherwise,
                    // this is an unnecessary performance penalty.
                    self.byte.count += 1;
                }
                Some(value)
            }
        }
    };
}

/// Create base methods for the ByteIter block of a skip iterator.
macro_rules! skip_iterator_byteiter_base {
    ($format:ident, $mask:ident) => {
        // It's contiguous if we don't skip over any values.
        // IE, the digit separator flags for the iterator over
        // the digits doesn't skip any values.
        const IS_CONTIGUOUS: bool = $format & flags::$mask == 0;

        #[inline]
        fn as_ptr(&self) -> *const u8 {
            self.byte.as_ptr()
        }

        #[inline]
        fn as_slice(&self) -> &'a [u8] {
            self.byte.as_slice()
        }

        #[inline]
        fn length(&self) -> usize {
            self.byte.length()
        }

        #[inline]
        fn cursor(&self) -> usize {
            self.byte.cursor()
        }

        #[inline]
        unsafe fn set_cursor(&mut self, index: usize) {
            debug_assert!(index <= self.length());
            // SAFETY: safe if `index <= self.length()`.
            unsafe { self.byte.set_cursor(index) };
        }

        #[inline]
        fn current_count(&self) -> usize {
            self.byte.current_count()
        }

        #[inline]
        fn is_consumed(&mut self) -> bool {
            self.peek().is_none()
        }

        #[inline]
        fn is_done(&self) -> bool {
            self.byte.is_done()
        }

        #[inline]
        fn is_contiguous(&self) -> bool {
            Self::IS_CONTIGUOUS
        }

        #[inline]
        unsafe fn peek_unchecked(&mut self) -> <Self as Iterator>::Item {
            self.peek().unwrap()
        }

        #[inline]
        unsafe fn read_unchecked<V>(&self) -> V {
            debug_assert!(self.as_slice().len() >= mem::size_of::<V>());
            // SAFETY: safe as long as the slice has at least count elements.
            unsafe { self.byte.read_unchecked() }
        }

        #[inline]
        fn read<V>(&self) -> Option<V> {
            self.byte.read()
        }

        #[inline]
        unsafe fn step_by_unchecked(&mut self, count: usize) {
            debug_assert!(self.as_slice().len() >= count);
            // SAFETY: safe as long as `slc.len() >= count`.
            unsafe { self.byte.step_by_unchecked(count) }
        }
    };
}

/// Create impl ByteIter block for skip iterator.
macro_rules! skip_iterator_byteiter_impl {
    ($iterator:ident, $mask:ident, $i:ident, $l:ident, $t:ident, $c:ident) => {
        impl<'a: 'b, 'b, const FORMAT: u128> BytesIter<'a> for $iterator<'a, 'b, FORMAT> {
            skip_iterator_byteiter_base!(FORMAT, $mask);

            /// Peek the next value of the iterator, without consuming it.
            #[inline]
            fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
                let format = NumberFormat::<{ FORMAT }> {};
                const IL: u128 = flags::$i | flags::$l;
                const IT: u128 = flags::$i | flags::$t;
                const LT: u128 = flags::$l | flags::$t;
                const ILT: u128 = flags::$i | flags::$l | flags::$t;
                const IC: u128 = flags::$i | flags::$c;
                const LC: u128 = flags::$l | flags::$c;
                const TC: u128 = flags::$t | flags::$c;
                const ILC: u128 = IL | flags::$c;
                const ITC: u128 = IT | flags::$c;
                const LTC: u128 = LT | flags::$c;
                const ILTC: u128 = ILT | flags::$c;

                match format.digit_separator_flags() & flags::$mask {
                    0 => peek_noskip!(self),
                    flags::$i => peek_i!(self),
                    flags::$l => peek_l!(self),
                    flags::$t => peek_t!(self),
                    IL => peek_il!(self),
                    IT => peek_it!(self),
                    LT => peek_lt!(self),
                    ILT => peek_ilt!(self),
                    IC => peek_ic!(self),
                    LC => peek_lc!(self),
                    TC => peek_tc!(self),
                    ILC => peek_ilc!(self),
                    ITC => peek_itc!(self),
                    LTC => peek_ltc!(self),
                    ILTC => peek_iltc!(self),
                    _ => unreachable!(),
                }
            }
        }
    };
}

// INTEGER DIGITS ITERATOR
// -----------------------

skip_iterator!(IntegerBytesIterator, "Iterator that skips over digit separators in the integer.");
skip_iterator_impl!(IntegerBytesIterator, mantissa_radix);
skip_iterator_iterator_impl!(IntegerBytesIterator);
skip_iterator_byteiter_impl!(
    IntegerBytesIterator,
    INTEGER_DIGIT_SEPARATOR_FLAG_MASK,
    INTEGER_INTERNAL_DIGIT_SEPARATOR,
    INTEGER_LEADING_DIGIT_SEPARATOR,
    INTEGER_TRAILING_DIGIT_SEPARATOR,
    INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
);

// FRACTION DIGITS ITERATOR
// ------------------------

skip_iterator!(FractionBytesIterator, "Iterator that skips over digit separators in the fraction.");
skip_iterator_impl!(FractionBytesIterator, mantissa_radix);
skip_iterator_iterator_impl!(FractionBytesIterator);
skip_iterator_byteiter_impl!(
    FractionBytesIterator,
    FRACTION_DIGIT_SEPARATOR_FLAG_MASK,
    FRACTION_INTERNAL_DIGIT_SEPARATOR,
    FRACTION_LEADING_DIGIT_SEPARATOR,
    FRACTION_TRAILING_DIGIT_SEPARATOR,
    FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
);

// EXPONENT DIGITS ITERATOR
// ------------------------

skip_iterator!(ExponentBytesIterator, "Iterator that skips over digit separators in the exponent.");
skip_iterator_impl!(ExponentBytesIterator, exponent_radix);
skip_iterator_iterator_impl!(ExponentBytesIterator);
skip_iterator_byteiter_impl!(
    ExponentBytesIterator,
    EXPONENT_DIGIT_SEPARATOR_FLAG_MASK,
    EXPONENT_INTERNAL_DIGIT_SEPARATOR,
    EXPONENT_LEADING_DIGIT_SEPARATOR,
    EXPONENT_TRAILING_DIGIT_SEPARATOR,
    EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
);

// SPECIAL DIGITS ITERATOR
// -----------------------

skip_iterator!(
    SpecialBytesIterator,
    "Iterator that skips over digit separators in special floats."
);
skip_iterator_iterator_impl!(SpecialBytesIterator);

impl<'a: 'b, 'b, const FORMAT: u128> SpecialBytesIterator<'a, 'b, FORMAT> {
    is_digit_separator!(FORMAT);
}

impl<'a: 'b, 'b, const FORMAT: u128> BytesIter<'a> for SpecialBytesIterator<'a, 'b, FORMAT> {
    skip_iterator_byteiter_base!(FORMAT, SPECIAL_DIGIT_SEPARATOR);

    /// Peek the next value of the iterator, without consuming it.
    #[inline]
    fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        let format = NumberFormat::<{ FORMAT }> {};
        if format.special_digit_separator() {
            peek_iltc!(self)
        } else {
            peek_noskip!(self)
        }
    }
}
