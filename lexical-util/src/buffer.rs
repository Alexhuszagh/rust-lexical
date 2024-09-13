//! Specialized buffer traits.
//!
//! The traits are for iterables containing bytes, and provide optimizations
//! which then can be used for contiguous or non-contiguous iterables,
//! including containers or iterators of any kind.

use core::mem;

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

// NOTE: These two functions are taken directly from the Rust corelib.
//  https://doc.rust-lang.org/1.81.0/src/core/mem/maybe_uninit.rs.html#964

/// Assuming all the elements are initialized, get a slice to them.
///
/// # Safety
///
/// It is up to the caller to guarantee that the `MaybeUninit<T>` elements
/// really are in an initialized state. Calling this when the content is not
/// yet fully initialized causes undefined behavior.
///
/// See [`assume_init_ref`] for more details and examples.
///
/// [`assume_init_ref`]: mem::MaybeUninit::assume_init_ref
#[inline(always)]
pub const unsafe fn slice_assume_init<T>(slice: &[mem::MaybeUninit<T>]) -> &[T] {
    // SAFETY: casting `slice` to a `*const [T]` is safe since the caller guarantees that
    // `slice` is initialized, and `MaybeUninit` is guaranteed to have the same layout as `T`.
    // The pointer obtained is valid since it refers to memory owned by `slice` which is a
    // reference and thus guaranteed to be valid for reads.
    unsafe { &*(slice as *const [mem::MaybeUninit<T>] as *const [T]) }
}

// Assuming all the elements are initialized, get a mutable slice to them.
///
/// # Safety
///
/// It is up to the caller to guarantee that the `MaybeUninit<T>` elements
/// really are in an initialized state. Calling this when the content is
/// not yet fully initialized causes undefined behavior.
///
/// See [`assume_init_mut`] for more details and examples.
///
/// [`assume_init_mut`]: mem::MaybeUninit::assume_init_mut
#[inline(always)]
pub unsafe fn slice_assume_init_mut<T>(slice: &mut [mem::MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(slice as *mut [mem::MaybeUninit<T>] as *mut [T]) }
}
