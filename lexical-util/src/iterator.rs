//! Specialized iterator traits.
//!
//! The traits are for iterables containing bytes, and provide optimizations
//! which then can be used for contiguous or non-contiguous iterables,
//! including containers or iterators of any kind.

#![cfg(any(feature = "parse-floats", feature = "parse-integers"))]

use core::mem;

// Re-export our digit iterators.
use crate::error::Error;
use crate::format::NumberFormat;
#[cfg(not(feature = "format"))]
pub use crate::noskip::{AsBytes, Bytes};
use crate::result::Result;
#[cfg(feature = "format")]
pub use crate::skip::{AsBytes, Bytes};

/// Converts the sign parser to return a result and handles increment the
/// internal state.
///
/// 1. Parses the sign digit.
/// 2. Handles if positive signs before integers are not allowed.
/// 3. Handles negative signs if the type is unsigned.
/// 4. Handles if the sign is required, but missing.
/// 5. Handles if the iterator is empty, before or after parsing the sign.
/// 6. Handles if the iterator has invalid, leading zeros.
///
/// This does not handle missing digits: it is assumed the caller will.
///
/// It assumes the next digit is the sign character, that is,
/// leading and trailing digits **HAVE** been handled for non-
/// contiguous iterators.
macro_rules! read_sign {
    (
        $byte:ident,
        $index:ident,
        $is_signed:expr,
        $no_positive:expr,
        $required:expr,
        $invalid_positive:ident,
        $missing:ident $(,)?
    ) => {{
        let (is_negative, have_sign) = match $byte.get_buffer().get($index) {
            Some(&b'+') => (false, true),
            Some(&b'-') => (true, true),
            _ => (false, false),
        };
        match (is_negative, have_sign) {
            (false, true) if !$no_positive => {
                // SAFETY: We have at least 1 item left since we peaked a value
                unsafe { $byte.set_cursor($index + 1) };
                Ok(false)
            },
            (false, true) if $no_positive => Err(Error::$invalid_positive($byte.cursor())),
            (true, true) if $is_signed => {
                // SAFETY: We have at least 1 item left since we peaked a value
                unsafe { $byte.set_cursor($index + 1) };
                Ok(true)
            },
            _ if $required => Err(Error::$missing($byte.cursor())),
            _ => Ok(false),
        }
    }};
}

/// Parse the sign from the leading integer digits.
///
/// It assumes the next digit is the sign character, that is,
/// leading and trailing digits **HAVE** been handled for non-
/// contiguous iterators.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub(crate) fn read_integer_sign<const FORMAT: u128>(
    byte: &mut Bytes<'_, FORMAT>,
    index: usize,
    is_signed: bool,
) -> Result<bool> {
    let format = NumberFormat::<FORMAT> {};
    read_sign!(
        byte,
        index,
        is_signed,
        format.no_positive_mantissa_sign(),
        format.required_mantissa_sign(),
        InvalidPositiveSign,
        MissingSign,
    )
}

/// Parse the sign from the leading mantissa digits.
///
/// It assumes the next digit is the sign character, that is,
/// leading and trailing digits **HAVE** been handled for non-
/// contiguous iterators.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub(crate) fn read_mantissa_sign<const FORMAT: u128>(
    byte: &mut Bytes<'_, FORMAT>,
    index: usize,
) -> Result<bool> {
    let format = NumberFormat::<FORMAT> {};
    read_sign!(
        byte,
        index,
        true,
        format.no_positive_mantissa_sign(),
        format.required_mantissa_sign(),
        InvalidPositiveMantissaSign,
        MissingMantissaSign,
    )
}

/// Parse the sign from the leading exponent digits.
///
/// It assumes the next digit is the sign character, that is,
/// leading and trailing digits **HAVE** been handled for non-
/// contiguous iterators.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub(crate) fn read_exponent_sign<const FORMAT: u128>(
    byte: &mut Bytes<'_, FORMAT>,
    index: usize,
) -> Result<bool> {
    let format = NumberFormat::<FORMAT> {};
    read_sign!(
        byte,
        index,
        true,
        format.no_positive_exponent_sign(),
        format.required_exponent_sign(),
        InvalidPositiveExponentSign,
        MissingExponentSign,
    )
}

/// A trait for working with iterables of bytes.
///
/// These iterators can either be contiguous or not contiguous and provide
/// methods for reading data and accessing underlying data. The readers
/// can either be contiguous or non-contiguous, although performance and
/// some API methods may not be available for both.
///
/// # Safety
///
/// Safe if [`set_cursor`] is set to an index <= [`buffer_length`], so no
/// out-of-bounds reads can occur. Also, [`get_buffer`] must return a slice of
/// initialized bytes. The caller must also ensure that any calls that increment
/// the cursor, such as [`step_by_unchecked`], [`step_unchecked`], and
/// [`peek_many_unchecked`] never exceed [`buffer_length`] as well.
///
/// [`set_cursor`]: `Iter::set_cursor`
/// [`buffer_length`]: `Iter::buffer_length`
/// [`get_buffer`]: `Iter::get_buffer`
/// [`step_by_unchecked`]: `Iter::step_by_unchecked`
/// [`step_unchecked`]: `Iter::step_unchecked`
/// [`peek_many_unchecked`]: `Iter::peek_many_unchecked`
pub unsafe trait Iter<'a> {
    /// Determine if the buffer is contiguous in memory.
    const IS_CONTIGUOUS: bool;

    // CURSORS
    // -------

    /// Get a ptr to the current start of the buffer.
    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    /// Get a slice to the current start of the buffer.
    #[inline(always)]
    fn as_slice(&self) -> &'a [u8] {
        debug_assert!(self.cursor() <= self.buffer_length());
        // SAFETY: safe since index must be in range.
        unsafe { self.get_buffer().get_unchecked(self.cursor()..) }
    }

    /// Get a slice to the full underlying contiguous buffer,
    fn get_buffer(&self) -> &'a [u8];

    /// Get the total number of elements in the underlying buffer.
    #[inline(always)]
    fn buffer_length(&self) -> usize {
        self.get_buffer().len()
    }

    /// Get if no bytes are available in the buffer.
    ///
    /// This operators on the underlying buffer: that is,
    /// it returns if [`as_slice`] would return an empty slice.
    ///
    /// [`as_slice`]: Iter::as_slice
    #[inline(always)]
    fn is_buffer_empty(&self) -> bool {
        self.cursor() >= self.get_buffer().len()
    }

    /// Get the current index of the iterator in the slice.
    fn cursor(&self) -> usize;

    /// Set the current index of the iterator in the slice.
    ///
    /// This is **NOT** the current position of the iterator,
    /// since iterators may skip digits: this is the cursor
    /// in the underlying buffer. For example, if `slc[2]` is
    /// skipped, `set_cursor(3)` would be the 3rd element in
    /// the iterator, not the 4th.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.buffer_length()`. Although this
    /// won't affect safety, the caller also should be careful it
    /// does not set the cursor within skipped characters
    /// since this could affect correctness: an iterator that
    /// only accepts non-consecutive digit separators would
    /// pass if the cursor was set between the two.
    unsafe fn set_cursor(&mut self, index: usize);

    // PROPERTIES

    /// Determine if the iterator is contiguous.
    ///
    /// For digits iterators, this may mean that only that component
    /// of the number if contiguous, but the rest is not: that is,
    /// digit separators may be allowed in the integer but not the
    /// fraction, and the integer iterator would be contiguous but
    /// the fraction would not.
    #[inline(always)]
    fn is_contiguous(&self) -> bool {
        Self::IS_CONTIGUOUS
    }

    /// Get a value at an index without stepping to it from the underlying
    /// buffer.
    ///
    /// This does **NOT** skip digits, and directly fetches the item
    /// from the underlying buffer, relative to the current cursor.
    #[inline(always)]
    fn get(&self, index: usize) -> Option<&'a u8> {
        self.get_buffer().get(self.cursor() + index)
    }

    /// Check if two values are equal, with optional case sensitivity.
    #[inline(always)]
    fn is_value_equal(lhs: u8, rhs: u8, is_cased: bool) -> bool {
        if is_cased {
            lhs == rhs
        } else {
            lhs.eq_ignore_ascii_case(&rhs)
        }
    }

    /// Get the next value available without consuming it.
    ///
    /// This does **NOT** skip digits, and directly fetches the item
    /// from the underlying buffer.
    #[inline(always)]
    fn first(&self) -> Option<&'a u8> {
        self.get(0)
    }

    /// Check if the next element is a given value, in a case-
    /// sensitive manner.
    #[inline(always)]
    fn first_is_cased(&self, value: u8) -> bool {
        Some(&value) == self.first()
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    fn first_is_uncased(&self, value: u8) -> bool {
        if let Some(&c) = self.first() {
            c.eq_ignore_ascii_case(&value)
        } else {
            false
        }
    }

    /// Check if the next item in buffer is a given value with optional case
    /// sensitivity.
    #[inline(always)]
    fn first_is(&self, value: u8, is_cased: bool) -> bool {
        if is_cased {
            self.first_is_cased(value)
        } else {
            self.first_is_uncased(value)
        }
    }

    // STEP BY
    // -------

    /// Advance the internal slice by `N` elements.
    ///
    /// This does not advance the iterator by `N` elements for
    /// non-contiguous iterators: this just advances the internal,
    /// underlying buffer. This is useful for multi-digit optimizations
    /// for contiguous iterators.
    ///
    /// This does not increment the count of items: returns: this only
    /// increments the index, not the total digits returned. You must use
    /// this carefully: if stepping over a digit, you must then call
    /// [`increment_count`] afterwards or else the internal count will
    /// be incorrect.
    ///
    /// This does not skip digit separators and so if used incorrectly,
    /// the buffer may be in an invalid state, such as setting the next
    /// return value to a digit separator it should have skipped.
    ///
    /// [`increment_count`]: DigitsIter::increment_count
    ///
    /// # Panics
    ///
    /// This will panic if the buffer advances for non-contiguous
    /// iterators if the current byte is a digit separator, or if the
    /// count is more than 1.
    ///
    /// # Safety
    ///
    /// As long as the iterator is at least `N` elements, this
    /// is safe.
    unsafe fn step_by_unchecked(&mut self, count: usize);

    /// Advance the internal slice by 1 element.
    ///
    /// This does not increment the count of items: returns: this only
    /// increments the index, not the total digits returned. You must
    /// use this carefully: if stepping over a digit, you must then call
    /// [`increment_count`] afterwards or else the internal count will
    /// be incorrect.
    ///
    /// This does not skip digit separators and so if used incorrectly,
    /// the buffer may be in an invalid state, such as setting the next
    /// return value to a digit separator it should have skipped.
    ///
    /// [`increment_count`]: DigitsIter::increment_count
    ///
    /// # Panics
    ///
    /// This will panic if the buffer advances for non-contiguous
    /// iterators if the current byte is a digit separator.
    ///
    /// # Safety
    ///
    /// Safe as long as the iterator is not empty.
    #[inline(always)]
    unsafe fn step_unchecked(&mut self) {
        debug_assert!(!self.as_slice().is_empty());
        // SAFETY: safe if `self.index < self.buffer_length()`.
        unsafe { self.step_by_unchecked(1) };
    }

    // READ
    // ----

    /// Read a value of a difference type from the iterator.
    ///
    /// This does **not** advance the internal state of the iterator.
    /// This can only be implemented for contiguous iterators: non-
    /// contiguous iterators **MUST** panic.
    ///
    /// # Panics
    ///
    /// If the iterator is a non-contiguous iterator.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V. This must be unimplemented for
    /// non-contiguous iterators.
    #[inline(always)]
    unsafe fn peek_many_unchecked<V>(&self) -> V {
        unimplemented!();
    }

    /// Try to read a the next four bytes as a u32.
    ///
    /// This does not advance the internal state of the iterator.
    /// This will only return a value for contiguous iterators.
    #[inline(always)]
    fn peek_u32(&self) -> Option<u32> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<u32>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read. u32 is valid for all bit patterns
            unsafe { Some(self.peek_many_unchecked()) }
        } else {
            None
        }
    }

    /// Try to read the next eight bytes as a u64.
    ///
    /// This does not advance the internal state of the iterator.
    #[inline(always)]
    fn peek_u64(&self) -> Option<u64> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<u64>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read. u64 is valid for all bit patterns
            unsafe { Some(self.peek_many_unchecked()) }
        } else {
            None
        }
    }

    /// Parse the sign from an integer (not for floats).
    ///
    /// If this allows leading digit separators, it will handle
    /// those internally and advance the state as needed. This
    /// returned if the value is negative, or any error found when parsing the
    /// sign. This does not handle missing digits: it is assumed the caller
    /// will. This internally increments the count to right after the sign.
    ///
    /// The default implementation does not support digit separators.
    ///
    /// 1. Parses the sign digit.
    /// 2. Handles if positive signs are not allowed.
    /// 3. Handles negative signs if the type is unsigned.
    /// 4. Handles if the sign is required, but missing.
    /// 5. Handles if the iterator is empty, before or after parsing the sign.
    /// 6. Handles if the iterator has invalid, leading zeros.
    fn read_integer_sign(&mut self, is_signed: bool) -> Result<bool>;

    /// Parse the sign from a mantissa (only for floats).
    ///
    /// If this allows leading digit separators, it will handle
    /// those internally and advance the state as needed. This
    /// returned if the value is negative, or any error found when parsing the
    /// sign. This does not handle missing digits: it is assumed the caller
    /// will. This internally increments the count to right after the sign.
    ///
    /// The default implementation does not support digit separators.
    ///
    /// 1. Parses the sign digit.
    /// 2. Handles if positive signs are not allowed.
    /// 3. Handles negative signs if the type is unsigned.
    /// 4. Handles if the sign is required, but missing.
    /// 5. Handles if the iterator is empty, before or after parsing the sign.
    /// 6. Handles if the iterator has invalid, leading zeros.
    fn read_mantissa_sign(&mut self) -> Result<bool>;

    /// Parse the sign from an exponent.
    ///
    /// If this allows leading digit separators, it will handle
    /// those internally and advance the state as needed. This
    /// returned if the value is negative, or any error found when parsing the
    /// sign. This does not handle missing digits: it is assumed the caller
    /// will. This internally increments the count to right after the sign.
    ///
    /// The default implementation does not support digit separators.
    ///
    /// 1. Parses the sign digit.
    /// 2. Handles if positive signs are not allowed.
    /// 3. Handles negative signs if the type is unsigned.
    /// 4. Handles if the sign is required, but missing.
    /// 5. Handles if the iterator is empty, before or after parsing the sign.
    /// 6. Handles if the iterator has invalid, leading zeros.
    fn read_exponent_sign(&mut self) -> Result<bool>;

    /// Read the base prefix, if present, returning if the base prefix
    /// was present.
    ///
    /// If the base prefix was not present, it does not consume any
    /// leading zeroes or digit separators, so they can be processed afterwards.
    /// Otherwise, it advances the iterator state to the end of the base
    /// prefix, including consuming any trailing digit separators.
    ///
    /// Any caller that consumes leading digit separators will need
    /// to ignore it if base prefix trailing digit separators are enabled.
    fn read_base_prefix(&mut self) -> bool;

    /// Read the base suffix, if present, returning if the base suffix
    /// was present.
    ///
    /// If the base suffix was not present, it does not consume any
    /// digits or digit separators, so the total digit count is valid.
    /// Otherwise, it advances the iterator state to the end of the base
    /// suffix, including consuming any trailing digit separators.
    fn read_base_suffix(&mut self, has_exponent: bool) -> bool;
}

/// Iterator over a contiguous block of bytes.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
pub trait DigitsIter<'a>: Iterator<Item = &'a u8> + Iter<'a> {
    /// Get if the iterator cannot return any more elements.
    ///
    /// This may advance the internal iterator state, but not
    /// modify the next returned value.
    ///
    /// If this is an iterator, this is based on the number of items
    /// left to be returned. We do not necessarly know the length of
    /// the buffer. If this is a non-contiguous iterator, this **MUST**
    /// advance the state until it knows a value can be returned.
    ///
    /// Any incorrect implementations of this affect all safety invariants
    /// for the rest of the trait. For contiguous iterators, this can be
    /// as simple as checking if `self.cursor >= self.slc.len()`, but for
    /// non-contiguous iterators you **MUST** advance to the next element
    /// to be returned, then check to see if a value exists. The safest
    /// implementation is always to check if `self.peek().is_none()` and
    /// ensure [`peek`] is always safe.
    ///
    /// If you would like to see if the cursor is at the end of the buffer,
    /// see [`is_buffer_empty`] instead.
    ///
    /// [`is_buffer_empty`]: Iter::is_buffer_empty
    /// [`peek`]: DigitsIter::peek
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)] // reason="required for peeking next item"
    fn is_consumed(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Increment the number of digits that have been returned by the iterator.
    ///
    /// For contiguous iterators, this is a no-op. For non-contiguous iterators,
    /// this increments the count by 1.
    fn increment_count(&mut self);

    /// Get the number of digits the iterator has encountered.
    ///
    /// This is always relative to the start of the iterator: recreating
    /// the iterator will reset this count. The absolute value of this
    /// is not defined: only the relative value between 2 calls. For
    /// consecutive digit separators, this is based on the index in the
    /// buffer. For non-consecutive iterators, the count is internally
    /// incremented.
    fn digits(&self) -> usize;

    /// Get number of digits returned relative to a previous state.
    ///
    /// This allows you to determine how many digits were returned
    /// since a previous state, but is meant to be strongish-ly
    /// typed so the caller knows it only works within a single
    /// iterator. Calling this on an iterator other than the one
    /// used at the start may lead to unpredictable results.
    #[inline(always)]
    fn digits_since(&self, start: usize) -> usize {
        self.digits() - start
    }

    /// Peek the next value of the iterator, without consuming it.
    ///
    /// Note that this can modify the internal state, by skipping digits
    /// for iterators that find the first non-zero value, etc. We optimize
    /// this for the case where we have contiguous iterators, since
    /// non-contiguous iterators already have a major performance penalty.
    ///
    /// That is, say we have the following buffer and are skipping `_`
    /// characters, peek will advance the internal index to `2` if it
    /// can skip characters there.
    ///
    /// +---+---+---+---+---+        +---+---+---+---+
    /// | _ | 2 | _ | _ | 3 |   ->   | 2 | _ | _ | 3 |
    /// +---+---+---+---+---+        +---+---+---+---+
    ///
    /// For implementation reasons, where digit separators may not be
    /// allowed afterwards that character, it must stop right there.
    fn peek(&mut self) -> Option<Self::Item>;

    /// Peek the next value of the iterator, and step only if it exists.
    ///
    /// This will always advance to one byte past the peek value, since
    /// we may need to know internally if the next character is a digit
    /// separator.
    ///
    /// That is, say we have the following buffer and are skipping `_`
    /// characters, peek will advance the internal index to `_` if it
    /// can skip characters there.
    ///
    /// +---+---+---+---+---+        +---+---+---+
    /// | _ | 2 | _ | _ | 3 |   ->   | _ | _ | 3 |
    /// +---+---+---+---+---+        +---+---+---+
    #[inline(always)]
    fn try_read(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.peek() {
            // SAFETY: the slice cannot be empty because we peeked a value.
            unsafe { self.step_unchecked() };
            Some(value)
        } else {
            None
        }
    }

    /// Check if the next element is a given value, in a case-
    /// sensitive manner.
    #[inline(always)]
    fn peek_is_cased(&mut self, value: u8) -> bool {
        Some(&value) == self.peek()
    }

    /// Check if the next element is a given value without case
    /// sensitivity.
    #[inline(always)]
    fn peek_is_uncased(&mut self, value: u8) -> bool {
        if let Some(&c) = self.peek() {
            c.eq_ignore_ascii_case(&value)
        } else {
            false
        }
    }

    /// Check if the next element is a given value with optional case
    /// sensitivity.
    #[inline(always)]
    fn peek_is(&mut self, value: u8, is_cased: bool) -> bool {
        if is_cased {
            self.peek_is_cased(value)
        } else {
            self.peek_is_uncased(value)
        }
    }

    /// Peek the next value and consume it if the read value matches the
    /// expected one using a custom predicate.
    #[inline(always)]
    fn read_if<Pred: FnOnce(u8) -> bool>(&mut self, pred: Pred) -> Option<u8> {
        // NOTE: This was implemented to remove usage of unsafe throughout to code
        // base, however, performance was really not up to scratch. I'm not sure
        // the cause of this.
        if let Some(&peeked) = self.peek() {
            if pred(peeked) {
                // SAFETY: the slice cannot be empty because we peeked a value.
                unsafe { self.step_unchecked() };
                Some(peeked)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Read a value if the value matches the provided one, in a case-
    /// sensitive manner.
    #[inline(always)]
    fn read_if_value_cased(&mut self, value: u8) -> Option<u8> {
        if self.peek() == Some(&value) {
            // SAFETY: the slice cannot be empty because we peeked a value.
            unsafe { self.step_unchecked() };
            Some(value)
        } else {
            None
        }
    }

    /// Read a value if the value matches the provided one without case
    /// sensitivity.
    #[inline(always)]
    fn read_if_value_uncased(&mut self, value: u8) -> Option<u8> {
        self.read_if(|x| x.eq_ignore_ascii_case(&value))
    }

    /// Read a value if the value matches the provided one, with optional
    /// case sensitivity.
    #[inline(always)]
    fn read_if_value(&mut self, value: u8, is_cased: bool) -> Option<u8> {
        if is_cased {
            self.read_if_value_cased(value)
        } else {
            self.read_if_value_uncased(value)
        }
    }

    /// Skip zeros from the start of the iterator.
    #[inline(always)]
    fn skip_zeros(&mut self) -> usize {
        let start = self.digits();
        while self.read_if_value_cased(b'0').is_some() {
            self.increment_count();
        }
        self.digits_since(start)
    }

    /// Determine if the character is a digit.
    fn is_digit(&self, value: u8) -> bool;
}
