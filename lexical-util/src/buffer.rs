//! Specialized buffer traits.
//!
//! The traits are for iterables containing bytes, and provide optimizations
//! which then can be used for contiguous or non-contiguous iterables,
//! including containers or iterators of any kind.

#![cfg(feature = "parse")]

/// A trait for working with iterables of bytes.
///
/// These buffers can either be contiguous or not contiguous and provide
/// methods for reading data and accessing underlying data. The readers
/// can either be contiguous or non-contiguous, although performance and
/// some API methods may not be available for both.
pub trait Buffer<'a> {
    /// Determine if the buffer is contiguous in memory.
    const IS_CONTIGUOUS: bool;

    /// Get a ptr to the current start of the buffer.
    fn as_ptr(&self) -> *const u8;

    /// Get a slice to the current start of the buffer.
    fn as_slice(&self) -> &'a [u8];

    /// Get if no bytes are available in the buffer.
    ///
    /// If this is an iterator, this is based left to be returned.
    /// We do not necessarly know the length of the buffer
    fn is_empty(&self) -> bool;

    /// Determine if the buffer is contiguous.
    #[inline(always)]
    fn is_contiguous(&self) -> bool {
        Self::IS_CONTIGUOUS
    }

    /// Peek the next value of the buffer, without checking bounds.
    ///
    /// # Safety
    ///
    /// Safe as long as there is at least a single valid value left in
    /// the buffer. Note that the behavior of this may lead to out-of-bounds
    /// access (for contiguous buffers) or panics (for non-contiguous
    /// buffers).
    unsafe fn first_unchecked(&self) -> &'a u8;

    /// Get the next value available without consuming it.
    #[inline(always)]
    fn first(&self) -> Option<&'a u8> {
        if !self.is_empty() {
            // SAFETY: safe since the buffer cannot be empty
            unsafe { Some(self.first_unchecked()) }
        } else {
            None
        }
    }

    /// Check if the next element is a given value.
    #[inline(always)]
    fn first_is(&self, value: u8) -> bool {
        if let Some(&c) = self.first() {
            c == value
        } else {
            false
        }
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    fn case_insensitive_first_is(&self, value: u8) -> bool {
        if let Some(&c) = self.first() {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }
}
