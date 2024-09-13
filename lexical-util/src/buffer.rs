//! Specialized buffer traits.
//!
//! The traits are for iterables containing bytes, and provide optimizations
//! which then can be used for contiguous or non-contiguous iterables,
//! including containers or iterators of any kind.

/// A trait for working with iterables of bytes.
///
/// These buffers can either be contiguous or not contiguous and provide
/// methods for reading data and accessing underlying data. The readers
/// can either be contiguous or non-contiguous, although performance and
/// some API methods may not be available for both.
///
/// # Safety
///
/// This trait is effectively safe but the implementor must guarantee that
/// `is_empty` is implemented correctly. For most implementations, this can
/// be `self.as_slice().is_empty()`, where `as_slice` is implemented as
/// `&self.bytes[self.index..]`.
#[cfg(feature = "parse")]
pub unsafe trait Buffer<'a> {
    /// Determine if the buffer is contiguous in memory.
    const IS_CONTIGUOUS: bool;

    /// Get a ptr to the current start of the buffer.
    fn as_ptr(&self) -> *const u8;

    /// Get a slice to the current start of the buffer.
    fn as_slice(&self) -> &'a [u8];

    /// Get if no bytes are available in the buffer.
    ///
    /// This operators on the underlying buffer: that is,
    /// it returns if [as_slice] would return an empty slice.
    ///
    /// [as_slice]: Buffer::as_slice
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

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
    ///
    /// # Safety
    ///
    /// An implementor must implement `is_empty` correctly in
    /// order to guarantee the traitt is safe: `is_empty` **MUST**
    /// ensure that one value remains, if the iterator is non-
    /// contiguous this means advancing the iterator to the next
    /// position.
    #[inline(always)]
    fn first(&self) -> Option<&'a u8> {
        if !self.is_empty() {
            // SAFETY: safe since the buffer cannot be empty as validated before.
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
